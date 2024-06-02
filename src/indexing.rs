use regex::Regex;
use rusqlite::params;
use std::collections::HashMap;
use scraper::{ElementRef, Html, Selector};
use crate::{sanitize::sanitize_string, DB_POOL};

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
    url: String,
    words: HashMap<String, usize>
}

impl IndexData {
    pub fn new(url: String) -> Self {
        Self { url, words: HashMap::new() }
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
}

/// Indexes websites and store results in the database, check the documentation
/// at `Indexing` to understand how it proceeds.
pub async fn index_url(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut scoreboard = IndexData::new(url.clone());
    let page = surf::get(url.clone()).await?.body_string().await?;
    let dom = Html::parse_fragment(&page);
    let conn = DB_POOL.clone().get().unwrap();

    let title_selector = Selector::parse("title").unwrap();
    let desc_selector = Selector::parse("meta[name='description']").unwrap();
    let some_selector = Selector::parse("p, h1, h2, h3, h4, h5, span").unwrap();

    // INFO: To get the first element out of a DOM selector, you somehow have to
    // call `next`.

    if let Some(title) = dom.select(&title_selector).next() {
        let title_content = title.first_child().unwrap()
            .value().as_text().unwrap()
            .to_string();
        scoreboard.incr_score(vec![title_content], 10);
    }
    if let Some(desc) = dom.select(&desc_selector).next() {
        let desc_content = desc.attr("content").unwrap().to_string();
        scoreboard.incr_score(vec![desc_content], 5);
    }
    let p_content = dom.select(&some_selector)
        .into_iter()
        .map(|c| get_all_texts(c))
        .flatten()
        .collect::<Vec<_>>();

    scoreboard.incr_score(p_content, 1);
    scoreboard.words
        .keys()
        .for_each(|word| {
            
            // INFO: To make sure no data is overwritten, we increment score
            // from previous calls when indexing. This may cause issues on 
            // reindexing.
            // TODO: Create a `flush_previous_index_data` function.
            
            let score = scoreboard.words.get(word).unwrap();
            if let Err(error) = conn.execute_batch(&format!("
                BEGIN;
                CREATE TABLE IF NOT EXISTS _{word} (url text PRIMARY KEY, score int);
                INSERT OR IGNORE INTO _{word} (URL, score) VALUES ('{url}', 0);
                UPDATE _{word} SET score = score + {score} WHERE url = '{url}';
                COMMIT;
            ")) {
                println!("Insertion error: {}", error.to_string());
            }
        });
    Ok(())
}
