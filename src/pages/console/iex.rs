use maud::{Markup, PreEscaped};

const JS_HYDRATION: &str = include_str!("./iex.js");

pub fn mod_indexing_expl() -> Markup {
    html! {
        script { (PreEscaped(JS_HYDRATION)) } 
        div class="mod_container" {
            div class="mod_header" {
                h3 class="mod_title" { "Indexing Explorer" }
            }
            div class="mod_container" {
                p {
                    "URLs in queue: ";
                    span id="ie__queue_length" { "?" }
                }
                p {
                    "Indexed URLs: ";
                    span id="ie__indexed_urls" { "?" }
                }
                form action="/api/get_queue" {
                    input type="submit" value="See queue snapshot";
                }
            }
        }
    }
}
