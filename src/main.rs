#![feature(thread_sleep_until)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;


mod sanitize;
mod indexer;
mod searching;
mod templates;
mod macros;
mod db;
mod error;
mod data_pool;
#[cfg(feature = "debug")] mod debug;

use std::env::args;

use db::local::{read_lines, write_lines};
use debug::routes::toggle_queue_bot;
use maud::Markup;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rocket::{form::validate::{Contains, Len}, fs::{relative, FileServer}, serde::json::Json};
use searching::feeling_lucky;
use templates::{indexing::indexing_page, search::search_result_page};
use indexer::url::QueueBot;
use indexer::sitemaps::SitemapBot;

lazy_static! {
    static ref DB_POOL: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file("index_db.db")
            .with_init(|c| c.execute_batch("
                PRAGMA synchronous = off;
                PRAGMA encoding = 'UTF-16';
                PRAGMA journal_mode = WAL;
            "));
        Pool::new(manager).unwrap()
    };
    static ref SITEMAP_BOT: SitemapBot = SitemapBot::init();
    static ref QUEUE_BOT: QueueBot = QueueBot::init();
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

#[post("/index/urls", data = "<url_list>")]
fn index_websites(url_list: Json<Vec<String>>) -> Markup {
    QUEUE_BOT.queue_url(url_list.0);
    indexing_page()
}

/// It's important to submit a domain to this route as `RobotsDefinition` will
/// not be able in every scenario to use a URL properly and is intended to use
/// a domain name.
#[post("/index/from_robots_txt?<domain>")]
async fn index_websites_from_robots(domain: String) -> Markup {
    use indexer::robots::RobotsDefinition;

    let robots_data = RobotsDefinition::from_domain(domain).await.unwrap();
    robots_data.db_save().unwrap();
    for sitemap_url in robots_data.sitemaps {
        SITEMAP_BOT.queue_sitemap(sitemap_url);
    }
    indexing_page()
}

#[rocket::main]
async fn main() -> () {
    let cargs = args().collect::<Vec<String>>();

    db::sites::init_table().expect("Failed to init 'sites' table.");
    db::domains::init_table().expect("Failed to init 'domains' table.");
    QUEUE_BOT.thread_bot();
    SITEMAP_BOT.thread_bot();

    if !cargs.contains("--no-queue-recover".to_string()) {
        let rurls = read_lines("./runtime/queue").unwrap_or(vec![]);

        println!("[QUEUE] Recovered {} URLs", rurls.len());
        QUEUE_BOT.queue_url(rurls);
    }

    let _ = rocket::build()
        .mount("/", routes![
            index_websites, 
            search_query, 
            search_default_ui,
            index_websites_from_robots
        ])
        .mount("/debug", routes![toggle_queue_bot])
        .mount("/static", FileServer::from(relative!("/static")))
        .launch()
        .await;

    write_lines("./runtime/queue", QUEUE_BOT.get_remaining_urls())
        .expect("werr");
}
