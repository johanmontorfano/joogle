use maud::Markup;

#[cfg(feature = "auto_queue")]
#[cfg(feature = "debug")]
#[post("/queue_bot?<enabled>")]
pub fn toggle_queue_bot(enabled: bool) -> Markup {
    use maud::html;
    use crate::QUEUE_BOT;

    *QUEUE_BOT.is_paused.lock().unwrap() = enabled;
    html! { p {(format!("Queue is {}", enabled))} }
}
