use maud::Markup;

pub fn mod_submit_link() -> Markup {
    html! {
        div class="mod_container" {
            div class="mod_header" {
                h3 class="mod_title" { "Link Submitter" }
                p class="mod_description" {
                    "Use it to submit a link to queue"
                }
            }
            div class="mod_body" {
                form action="/api/submit_link" {
                    label { "URL" }
                    input type="url" name="url" placeholder="Link" required;
                    label { "Priority ?" }
                    input type="checkbox" name="priority" required;
                    input type="submit" value="Submit link";
                }
            }
        }
    }
}
