use maud::{html, Markup, DOCTYPE};

/// Renders the indexing page.
pub fn indexing_page(proc_results: Vec<(String, String)>) -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "JOOGLE INDEXING" } 
            link rel="stylesheet" href="/static/global.css";
        }
        body {
            @for res in proc_results {
                p { (res.0) ": " (res.1) }
            }
        }
    }
}
