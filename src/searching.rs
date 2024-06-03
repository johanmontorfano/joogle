use std::collections::HashMap;
use crate::{sanitize::sanitize_string, DB_POOL};

/// From a HashMap<String, usize>, this function returns the keys ordered by 
/// their descending corresponding value.
fn get_desc_hash_map_keys(
    from: &mut HashMap<String, (f64, String, String)>
) -> Vec<(String, String, String)> {
    let mut vec: Vec<(&String, &(f64, String, String))> = from.iter().collect();
    vec.sort_by(|a, b| b.1.0.total_cmp(&a.1.0));
    vec.into_iter()
        .map(|(key, data)| (key.clone(), data.1.clone(), data.2.clone()))
        .collect()
}

/// Find matching results for a specific query by decomposing a query string 
/// into a list of words, and looking at which websites have the best cumulative
/// score.
/// INFO: This technique is meant to change, read the README to learn more.
pub fn feeling_lucky(query: String) -> Vec<(String, String, String)> {
    let mut scoreboard: HashMap<String, usize> = HashMap::new();
    let sanitized_query = sanitize_string(query);
    let conn = DB_POOL.clone().get().unwrap();

    // It's important to understand that we need to limit the number of URL 
    // results we get out of a word table because we do not need an infinite 
    // number of results.
    // TODO: If pagination can be implemented, it should be implemented here
    // to remove this query size limit.
    sanitized_query.iter().for_each(|w| {
        let select_stmt = conn.prepare(&format!("
            SELECT url, score FROM w_{w} 
            ORDER BY score DESC
            LIMIT 100
        "));
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

    // We load each website TTR score and multiply it's query dependant score by
    // it.
    let mut final_scoreboard = scoreboard.keys().into_iter().map(|url| {
        let mut select_stmt = conn.prepare(&format!("
            SELECT ttr, title, description FROM sites
            WHERE url = '{url}'
        ")).unwrap();
        let site_list = select_stmt
            .query_map([], |row| Ok((
                row.get::<usize, f64>(0).unwrap(),
                row.get::<usize, String>(1).unwrap(),
                row.get::<usize, String>(2).unwrap()
            )))
            .unwrap()
            .map(|r| r.unwrap())
            .collect::<Vec<(f64, String, String)>>();
        let site_data = site_list.get(0).unwrap();
        let site_score = scoreboard.get(url).unwrap();

        (
            url.clone(), 
            (
                (*site_score as f64) * site_data.0, 
                site_data.1.clone(), 
                site_data.2.clone()
            )
        ) 
    }).collect::<HashMap<String, (f64, String, String)>>();

    get_desc_hash_map_keys(&mut final_scoreboard)
}
