use std::collections::HashMap;

use regex::Regex;

/// Sanitize a string by removing every non alphanumeric char, lowercasing 
/// everyting, and splitting the string into a Vec of words.
/// WARN: This function can be optimized to be faster.
pub fn sanitize_string<T: std::fmt::Display>(s: T) -> Vec<String> {
    let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    let s = s.to_string().to_lowercase();

    re.split(&s)
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect()
}

/// Escape ' to avoid issues on SQL insertion.
pub fn sql_escape_ap(item: String) -> String {
    item.replace("'", "''")
}

/// Encode a User-Agent restrictions HashMap for storage in the database.
pub fn sql_encode_uas(source: HashMap<String, Vec<String>>) -> String {
    source.into_iter()
        .map(|(k, v)| (k, v.join(",")))
        .map(|(k, v)| format!("{k}:{v}"))
        .collect::<Vec<String>>()
        .join(" ")
}

/// Decode a User-Agent restrictions HashMap encoded by `sql_encode_uas`.
pub fn sql_decode_uas(source: String) -> HashMap<String, Vec<String>> {
    let hm_iter = source.split(" ")
        .map(|s| s.split_once(":").unwrap())
        .map(|(k, v)| (
            k.to_string(),
            v.split(",").map(|v| v.to_string()).collect()
        ));
    HashMap::from_iter(hm_iter)
}
