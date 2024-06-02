use std::collections::HashMap;
use crate::{sanitize::sanitize_string, DB_POOL};

const QUERY_SIZE_LIMIT: usize = 100;

/// From a HashMap<String, usize>, this function returns the keys ordered by 
/// their descending corresponding value.
fn get_desc_hash_map_keys(from: &mut HashMap<String, usize>) -> Vec<String> {
    let mut vec: Vec<(&String, &usize)> = from.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    vec.into_iter().map(|(key, _)| key.clone()).collect()
}

/// Find matching results for a specific query by decomposing a query string 
/// into a list of words, and looking at which websites have the best cumulative
/// score.
/// INFO: This technique is meant to change, read the README to learn more.
pub fn feeling_lucky(query: String) -> Vec<String> {
    let mut scoreboard: HashMap<String, usize> = HashMap::new();
    let sanitized_query = sanitize_string(query);
    let conn = DB_POOL.clone().get().unwrap();

    // It's important to understand that we need to limit the number of URL 
    // results we get out of a word table because we do not need an infinite 
    // number of results.
    // TODO: If pagination can be implemented, it should be implemented here
    // to remove this query size limit.
    sanitized_query.iter().for_each(|w| {
        let select_stmt = conn
            .prepare(&format!("SELECT url, score FROM _{w} 
                              ORDER BY score DESC
                              LIMIT {QUERY_SIZE_LIMIT}"));
        if select_stmt.is_err() {
            return ();
        }

        let mut select_stmt = select_stmt.unwrap();
        let score_iter = select_stmt.query_map([], |row| Ok((
            row.get::<usize, String>(0).unwrap(), 
            row.get::<usize, usize>(1).unwrap()
        ))).unwrap();

        score_iter.for_each(|row| {
            let (url, score) = row.unwrap();
            if scoreboard.contains_key(&url) {
                scoreboard.insert(
                    url.clone(), 
                    scoreboard.get(&url).unwrap() + score
                );
            } else {
                scoreboard.insert(url.clone(), score);
            }
        });
    });

    get_desc_hash_map_keys(&mut scoreboard)
}
