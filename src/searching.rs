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
    let mut scoreboard: HashMap<String, (f64, String, String)> = HashMap::new();
    let sanitized_query = sanitize_string(query);
    let mut conn = DB_POOL.clone().get().unwrap();

    // It's important to understand that we need to limit the number of URL 
    // results we get out of a word table because we do not need an infinite 
    // number of results.
    // TODO: If pagination can be implemented, it should be implemented here
    // to remove this query size limit.
    sanitized_query.iter().for_each(|w| { 
        let score_iter: Vec<(String, i64, f64, String, String)> =
            conn.query(&format!("
                SELECT w_{w}.url, 
                    w_{w}.score, 
                    sites.ttr, 
                    sites.title,
                    sites.description 
                FROM w_{w} 
                INNER JOIN sites ON w_{w}.url = sites.url
                ORDER BY w_{w}.score DESC
                LIMIT 100
            "), &[])
            .unwrap()
            .iter()
            .map(|row| (
                row.get::<usize, String>(0), 
                row.get::<usize, i64>(1),
                row.get::<usize, f64>(2),
                row.get::<usize, String>(3),
                row.get::<usize, String>(4)
            ))
            .collect(); 

        score_iter.iter().for_each(|row| {
            let (url, score, ttr, title, desc) = row;
            let score = *score as f64;

            if scoreboard.contains_key(url) {
                scoreboard.insert(
                    url.clone(), 
                    (
                        scoreboard.get(url).unwrap().0 + score * ttr, 
                        title.into(), 
                        desc.into()
                    )
                );
            } else {
                scoreboard.insert(
                    url.clone(), 
                    (
                        score * ttr, 
                        title.into(), 
                        desc.into()
                    ));
            }
        });
    });

    get_desc_hash_map_keys(&mut scoreboard)
}
