use maud::{html, Markup, DOCTYPE};

/// Renders the search result page.
pub fn search_result_page(
    query: String, results: Vec<(String, String, String)>
) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            link rel="stylesheet" href="/static/global.css";
            link rel="stylesheet" href="/static/search.css";
            title { "Results for: " (query) }
        }
        body {
            div id="search-area" {
                h1 { "Joogle" }
                form action="search" method="get" {
                    input type="text" name="q" placeholder="What's in your mind ?";
                    input type="submit" value="🔍";
                }
            }
            div id="results" {
                @for (url, title, desc) in results {
                    div {
                        a href=(url) {
                            (title)
                        }
                        span { (desc) }
                    }
                }
            }
        }
    }
}
