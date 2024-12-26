mod iex;
mod sublink;
use iex::mod_indexing_expl;
use maud::{Markup, DOCTYPE};
use sublink::mod_submit_link;

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
                div { (mod_submit_link()) }
                div { (mod_indexing_expl()) }
            }
        }
    }
}
