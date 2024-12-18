use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use tokio::runtime::Runtime;
use scraper::{ElementRef, Html, Selector};
use url::Url;
use crate::data_pool::DataPool;
use crate::{db, INDEXED_URLS_NB};
use crate::debug::gatherers::TimingGatherer;
use crate::error::StdError;
use crate::ifcfg;
use crate::sanitize::sanitize_string;
use crate::QUEUE_BOT;
use super::localization::{auto_choose_localization, get_localization};

/// Extract all texts from a root element.
pub fn get_all_texts(from: ElementRef) -> Vec<String> {
    from.children()
        .into_iter()
        .map(|c| {
            if c.value().is_text() {
                vec![c.value().as_text().unwrap().to_string()]
            } else {
                c.children().map(|c| {
                    if c.value().is_text() {
                        c.value().as_text().unwrap().to_string()
                    } else {
                        "".to_string()
                    }
                }).collect()
            }
        })
        .flatten()
        .filter(|f| f.len() > 0)
        .collect::<Vec<_>>()
}

/// Indexing data, before it can be stored on the database.
#[derive(Debug)]
pub struct IndexData {
    words: HashMap<String, usize>
}

impl IndexData {
    pub fn new() -> Self {
        Self { words: HashMap::new() }
    }

    /// Increase a word score from a string vec and a multiplier.
    pub fn incr_score(&mut self, lines: Vec<String>, rate: usize) {
        lines.iter().for_each(|line| {
            sanitize_string(line).iter().for_each(|w| {
                match self.words.get(w) {
                    Some(count) => { 
                        self.words.insert(w.into(), count + rate); 
                    }
                    None => { 
                        self.words.insert(w.into(), rate); 
                    }
                }
            });
        })
    }

    /// Increase a score from a selector.
    pub fn incr_score_selector(
        &mut self, dom: &Html, selector: Selector, rate: usize
    ) {
        let content = dom.select(&selector)
            .into_iter()
            .map(|c| get_all_texts(c))
            .flatten()
            .collect::<Vec<_>>();
        self.incr_score(content, rate);
    }

    /// Get the Type-Token Ratio to determine the quality of the page and add it
    /// to the website quality attribute.
    pub fn get_ttr(&self) -> f64 {
        let word_set: HashSet<String> = HashSet::from_iter(
            self.words.keys().into_iter().map(|w| w.to_string())
        );
        let word_count = self.words.keys().len();
        let ttr = word_set.len() as f64 / word_count as f64;

        if ttr.is_nan() { 0.1 } else { ttr }
    }
}

/// Indexes websites and store results in the database, check the documentation
/// at `Indexing` to understand how it proceeds.
pub async fn index_url(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut scoreboard = IndexData::new();
    let mut res = surf::get(url.clone()).await?;
    let page = res.body_string().await?;
    let dom = Html::parse_fragment(&page);
    let parsed_url = Url::parse(&url).unwrap();

    let title_selector = Selector::parse("title").unwrap();
    let desc_selector = Selector::parse("meta[name='description']").unwrap();
    let p_selector = Selector::parse("p, span").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let h1_selector = Selector::parse("h1").unwrap();
    let h2_selector = Selector::parse("h2").unwrap();
    let h3_selector = Selector::parse("h3").unwrap();
    let h4_selector = Selector::parse("h4").unwrap();
    let h5_selector = Selector::parse("h5").unwrap();

    let mut final_title = String::from("unnamed");
    let mut final_desc = String::from("No description.");

    if !res.status().is_success() {
        return StdError("Unsuccesful response code".into()).to_boxed_err()
    }
    // INFO: To get the first element out of a DOM selector, you somehow have to
    // call `next`.
    if let Some(title) = dom.select(&title_selector).next() {
        let title_content = title.first_child().unwrap()
            .value().as_text().unwrap()
            .to_string();
        scoreboard.incr_score(vec![title_content.clone()], 20);
        final_title = title_content;
    }
    if let Some(desc) = dom.select(&desc_selector).next() {
        let desc_content = desc.attr("content").unwrap().to_string();
        scoreboard.incr_score(vec![desc_content.clone()], 8);
        final_desc = desc_content;
    }

    // We create a record of the current domain to avoid any error related to
    // foreign keys referencing.
    db::domains::create_row_iff_empty(
        parsed_url.domain().unwrap().into(), 
        0, 
        HashMap::new(), 
        HashMap::new()
    )?;

    // We create a record of the current url on the database for later linking.
    db::sites::new_url_record(url.clone(), final_title, final_desc)?;
    scoreboard.incr_score_selector(&dom, p_selector, 1);
    scoreboard.incr_score_selector(&dom, h1_selector, 15);
    scoreboard.incr_score_selector(&dom, h2_selector, 10);
    scoreboard.incr_score_selector(&dom, h3_selector, 7);
    scoreboard.incr_score_selector(&dom, h4_selector, 5);
    scoreboard.incr_score_selector(&dom, h5_selector, 3);

    // We find other URLs we could index.
    if cfg!(feature = "auto_queue") {
        let new_links = dom.select(&a_selector)
            .into_iter()
            .map(|a| a.attr("href"))
            .filter(|r| r.is_some())
            .map(|a| QueueBot::ensure_url_format(
                url.clone(), a.unwrap().to_string()
            ))
            .filter(|r| r.is_ok())
            .map(|a| a.unwrap())
            .collect::<Vec<String>>();
        println!("Found automatically {} links to index.", new_links.len());
        QUEUE_BOT.queue_url(new_links);
    }

    // For each word, we link the current website to the word's table with it's
    // score with this word.
    scoreboard.words
        .keys()
        .for_each(|word| {
            let score = scoreboard.words.get(word).unwrap();
            let _ = db::_word::save_word_score(
                url.clone(), 
                word.clone(), 
                *score
            );
        });

    // The TTR and localization are saved alongside the site's data to determine the 
    // site's content quality.
    db::sites::update_site_ttr(&url, scoreboard.get_ttr())?;
    db::sites::update_site_loc(
        &url, 
        auto_choose_localization(get_localization(dom), scoreboard.get_ttr())
    )?; 

    Ok(())
}

pub struct QueueBot {
    data_pool: Arc<Mutex<DataPool<String>>>,
    pub is_paused: Arc<Mutex<bool>>
}

unsafe impl Send for QueueBot {}
unsafe impl Sync for QueueBot {}
impl QueueBot {
    pub fn init() -> Self {
        Self { 
            data_pool: Arc::new(Mutex::new(DataPool::init())),
            is_paused: Arc::new(Mutex::new(false))
        }
    }

    pub fn get_remaining_urls(&self) -> Vec<String> {
        self.data_pool.lock().unwrap().get_content()
    }

    /// This function MUST be called when auto-queuing to ensure only correcly
    /// formatted URLs are submitted.
    /// The source parameter is used to ensure that relative URLs gets their 
    /// absolute definition before being submitted to the queue.
    pub fn ensure_url_format(
        source: String, url: String
    ) -> Result<String, Box<dyn std::error::Error>> {
        let source_url = Url::parse(&source)?;
        let mut url = Url::parse(&url)?;

        if url.domain().is_none() {
            let _ = url.set_host(Some(&source_url.host().unwrap().to_string()));
            let _ = url.set_scheme(source_url.scheme());
        }

        Ok(url_escape::decode(&url.to_string()).to_string())
    }

    pub fn queue_url(&self, urls: Vec<String>) {
        self.data_pool.lock().unwrap().add_batch(urls);
    }

    /// Starts parallel indexing.
    pub fn thread_bot(&self) {
        let is_paused_clone = self.is_paused.clone();
        let pool_clone = self.data_pool.clone();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            let mut time_gatherer = { ifcfg!("debug", TimingGatherer::init()) };

            loop { 
                let mut guard = pool_clone.lock().unwrap();
                let url = guard.get_next();

                std::mem::drop(guard);

                ifcfg!("debug", time_gatherer.start_gathering());
                if *is_paused_clone.lock().unwrap() {
                    println!("QueueBot paused for 5 more seconds...");
                    thread::sleep(Duration::from_secs(5));
                    continue;
                }

                if let Some(u) = url {
                    rt.block_on(async {
                        println!("Indexing: {u}");
                        let msg = match index_url(u.clone()).await {
                            Ok(_) => {
                                unsafe { INDEXED_URLS_NB += 1; };
                                format!("Indexed: {u}")
                            }
                            Err(err) => format!("Error: {u} -> {err}")
                        };
                        println!("{msg}");
                    });
                    ifcfg!("debug", time_gatherer.action_done());
                    ifcfg!("debug", {
                        if time_gatherer.actions_done % 10 == 0 {
                            time_gatherer.log_gathered_data();
                        }
                    });
                }
            }
        });
    }
}
