use std::collections::{HashMap, HashSet};
use scraper::{ElementRef, Html, Selector};
use crate::{db, sanitize::sanitize_string, DB_POOL};

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

        word_set.len() as f64 / word_count as f64
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
