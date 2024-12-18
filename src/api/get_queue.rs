use std::ops::Add;

use rocket::serde::json::Json;
use crate::{INDEXED_URLS_NB, QUEUE_BOT};

#[get("/get_queue")]
pub fn get_queue() -> Json<Vec<String>> {
    return QUEUE_BOT.get_remaining_urls().into();
}

#[get("/get_queue_length")]
pub fn get_queue_length() -> String {
    return QUEUE_BOT.get_remaining_urls().len().to_string();
}

#[get("/get_indexed_urls")]
pub fn get_indexed_urls() -> String {
    return unsafe { INDEXED_URLS_NB.add(0).to_string() }
}
