mod iex;
use maud::{Markup, DOCTYPE};

/// To ensure ease of development, every part of the console will be a module,
/// making the console ui 100% modulable.
#[get("/search/console")]
pub fn console_ui() -> Markup {
    html! {
        html {
            (DOCTYPE)
            head {
                title { "Joogle Search Console" }
                link rel="stylesheet" href="/static/root.css";
                link rel="stylesheet" href="/static/dashboard.css";
            }
            body {
                h1 { "JOOGLE SEARCH CONSOLE" }
                (iex::mod_indexing_expl())
            }
        }
    }
}
