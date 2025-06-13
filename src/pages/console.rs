use rocket::response::content;

const SC_INDEX: &str = include_str!("../../static/sc-index.html");

/// To ensure ease of development, every part of the console will be a module,
/// making the console ui 100% modulable.
#[get("/search/console/<_..>")]
pub fn console_ui() -> content::RawHtml<&'static str> {
    content::RawHtml(SC_INDEX)
}
