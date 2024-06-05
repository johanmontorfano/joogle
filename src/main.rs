#![feature(thread_sleep_until)]
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
#[macro_use] extern crate rocket;

mod indexing;
mod sanitize;
mod searching;
mod templates;
mod db;

use lazy_static::lazy_static;
use maud::Markup;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rocket::{fs::{FileServer, relative}, serde::json::Json};
use searching::feeling_lucky;
use templates::{indexing::indexing_page, search::search_result_page};
use indexing::QueueBot;

lazy_static! {
    static ref DB_POOL: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file("index_db.db")
            .with_init(|c| c.execute_batch("
                PRAGMA synchronous = 3;
                PRAGMA encoding = 'UTF-16';
            "));
        Pool::new(manager).unwrap()
    };
    static ref QUEUE_BOT: indexing::QueueBot = {
        QueueBot::init()
    };
}

#[get("/search")]
fn search_default_ui() -> Markup {
    search_result_page("".into(), vec![])
}

#[get("/search?<q>")]
fn search_query(q: String) -> Markup {
    let results = feeling_lucky(q.clone());
    search_result_page(q, results)
}

#[post("/index", data = "<url_list>")]
async fn index_websites(url_list: Json<Vec<String>>) -> Markup {
    QUEUE_BOT.queue_url(url_list.0);
    indexing_page()
}

#[launch]
fn rocket() -> _ {
    db::sites::init_table().expect("Failed to init 'sites' table.");
    QUEUE_BOT.thread_bot();
    rocket::build()
        .mount("/", routes![index_websites, search_query, search_default_ui])
        .mount("/static", FileServer::from(relative!("/static")))
}
