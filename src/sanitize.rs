use regex::Regex;

/// Sanitize a string by removing every non alphanumeric char, lowercasing 
/// everyting, and splitting the string into a Vec of words.
/// WARN: This function can be optimized to be faster.
pub fn sanitize_string<T: std::fmt::Display>(s: T) -> Vec<String> {
    let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    let s = s.to_string().to_lowercase();

    re.split(&s)
        .filter(|w| !w.is_empty())
        .filter(|w| w.len() > 2)
        .map(|w| w.to_string())
        .collect()
}
