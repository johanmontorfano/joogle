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


