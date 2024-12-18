use maud::{Markup, DOCTYPE};
use crate::INDEXED_URLS_NB;

/// Renders the search result page. To avoid too much logic overhead, we
/// consider an empty query string as being set to print the Joogle's welcome
/// page.
pub fn search_result_page(
    query: String, 
    res: Vec<(String, String, String)>
) -> Markup {
    let is_dummy = query.is_empty();

    return html! {
        html {
            (DOCTYPE)
            head {
                meta charset="utf-8";
                link rel="stylesheet" href="/static/search.css";
                title { 
                    @if !is_dummy {
                        (query) " - "
                    }
                    "Joogle"
                } 
            }
            body {
                @if is_dummy {
                    (welcome())
                } @else {
                    (results(query, res))
                }
            }
        }
    }
}

fn welcome() -> Markup {
    let indexed;
    
    unsafe {
        indexed = INDEXED_URLS_NB;
    }
    html! {
        div class="big_search_container" {
            h1 class="big_title" { "JOOGLE" }
            form class="big_title_form" action="search" method="GET" {
                input type="text" name="q" placeholder="Type to search...";
                input type="submit" value="GO" hidden;
            }
        }
        footer class="stats_footer" {
            p { (indexed) " pages indexed" }
            a href="/search/console" { "Indexing console" }
        }
    }
}

fn results(query: String, res: Vec<(String, String, String)>) -> Markup {
    html! {
        header {
            p class="logo_like" { "JOOGLE" }
            form class="search_header" {
                input type="text" 
                    value=(query)
                    name="q" 
                    placeholder="Go on, search...";
                input type="submit" value="GO" hidden;
            }
        }
        div class="results_content" {
            @for result in res {
                div {
                    a href=(result.0) { (result.1) }
                    p { (result.2) }
                }
            }
        }
    }
}
