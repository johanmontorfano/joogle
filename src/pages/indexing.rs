use maud::{Markup, DOCTYPE};

/// Renders the indexing page.
pub fn indexing_page() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "JOOGLE INDEXING" } 
            link rel="stylesheet" href="/static/global.css";
        }
        body {
            p { "URLs have been queued !" }
        }
    }
}
