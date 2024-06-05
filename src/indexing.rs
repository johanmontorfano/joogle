use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}, thread, time::{Duration, SystemTime, UNIX_EPOCH}};
use rocket::form::validate::Len;
use tokio::runtime::Runtime;
use scraper::{ElementRef, Html, Selector};
use url::Url;
use crate::{db, sanitize::sanitize_string, QUEUE_BOT};

/// Extract all texts from a root element.
pub fn get_all_texts(from: ElementRef) -> Vec<String> {
    from.children()
        .into_iter()
        .map(|c| {
            if c.value().is_text() {
                vec![c.value().as_text().unwrap().to_string()]
            } else {
                c.children()
                    .map(|c| {
                        if c.value().is_text() {
                            c.value().as_text().unwrap().to_string()
                        } else {
                            "".to_string()
                        }
                    })
                    .collect()
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
        &mut self, dom: &Html, selector: Selector,rate: usize
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
    let page = surf::get(url.clone()).await?.body_string().await?;
    let dom = Html::parse_fragment(&page);

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

    // The TTR is saved alongside the site's data to determine the site's 
    // content quality.
    db::sites::update_site_ttr(url, scoreboard.get_ttr())?;

    Ok(())
}

/// The `QueueBot` will manage links to index by creating a queue of every 
/// website that should be indexed on a separate thread and with a separate
/// database connection.
pub struct QueueBot {
    queue: Arc<Mutex<Vec<String>>>
}

unsafe impl Send for QueueBot {}
unsafe impl Sync for QueueBot {}

impl QueueBot {
    pub fn init() -> Self {
        Self { queue: Arc::new(Mutex::new(vec![])) }
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
        let mut queue = self.queue.lock().unwrap();
        urls.iter().for_each(|url| {
            queue.push(url.into());
        });
    }

    /// Starts parallel indexing.
    pub fn thread_bot(&self) {
        let queue_clone = self.queue.clone();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            let mut total_processed_urls: u128 = 0;
            let mut total_processing_rate: u128 = 0;

            loop {
                let mut queue: Vec<String> = vec![];

                // We free the shared queue and drop it to make it available
                // as soon as possible.
                {
                    let mut guard = queue_clone.lock().unwrap();
                    std::mem::swap(&mut queue, &mut *guard);
                }
                for url in queue {
                    let start_at = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    rt.block_on(async {
                        println!("Indexing: {url}");
                        let msg = match index_url(url.clone()).await {
                            Ok(_) => format!("Indexed: {url}"),
                            Err(err) => format!("Error: {url} -> {err}")
                        };
                        println!("{msg}");
                    });
                    total_processed_urls += 1;
                    total_processing_rate += SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() - start_at;
                    println!("1 url / {} ms", 
                             total_processing_rate / total_processed_urls);
                }
            }
        });
    }
}
