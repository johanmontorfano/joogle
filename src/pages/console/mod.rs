use std::path::Path;

/// To ensure ease of development, every part of the console will be a module,
/// making the console ui 100% modulable.
#[get("/search/console")]
pub fn console_ui() -> String {
    "SEARCH CONSOLE APP SHOULD BE INCLUDED".to_string()
}
