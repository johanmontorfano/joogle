use std::ops::Add;
use rocket::serde::json::{Json, serde_json::json};
use crate::{INDEXED_URLS_NB, QUEUE_BOT};
use rocket::serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct IndexSysStatus {
    queue_length: u32,
    indexed_urls: u32
}

#[get("/index_sys_status")]
pub fn get_index_sys_status() -> Json<IndexSysStatus> {
    json!({
        "queue_length": QUEUE_BOT.get_remaining_urls().len(),
        "indexed_urls": unsafe { INDEXED_URLS_NB.into<u32>() }
    })
}
