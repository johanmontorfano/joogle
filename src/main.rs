#![feature(thread_sleep_until)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rusqlite;
#[macro_use] extern crate maud;
extern crate r2d2;
extern crate r2d2_sqlite;


mod sanitize;
mod indexer;
mod searching;
mod pages;
mod macros;
mod db;
mod error;
mod data_pool;
mod api;
mod models;
mod schema;
#[cfg(feature = "debug")] mod debug;

use std::env::{self, args};
use db::{local::{read_lines, write_lines}, sites::get_rows_number};
use debug::routes::toggle_queue_bot;
use maud::Markup;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rocket::{fairing::{Fairing, Info, Kind}, form::validate::Contains, fs::*, http::Header, serde::json::Json, Config, Request, Response};
use searching::feeling_lucky;
use pages::indexing::indexing_page;
use pages::search::search_result_page;
use pages::console::*;
use api::get_queue::*;
use api::ownership::*;
use indexer::url::QueueBot;
use indexer::sitemaps::SitemapBot;
use rocket_db_pools::Database;

static mut INDEXED_URLS_NB: isize = 0;

lazy_static! {
    static ref DB_POOL: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file("./runtime/index_db.db")
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

#[derive(Database)]
#[database("postgres")]
struct Pg(rocket_db_pools::diesel::PgPool);

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(
        &self,
        _: &'r Request<'_>,
        res: &mut Response<'r>
    ) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET"
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Credentials",
            "true"
        ));
    }
}

#[get("/")]
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
    let _ = dotenv_vault::dotenv();
    let cargs = args().collect::<Vec<String>>();
    let pg_figment = Config::figment().merge((
        "databases.postgres.url",
        env::var("PG_DIESEL_URL").expect("No Postgres URL specified.")
    ));

    db::sites::init_table().expect("Failed to init 'sites' table.");
    db::domains::init_table().expect("Failed to init 'domains' table.");
    QUEUE_BOT.thread_bot();
    SITEMAP_BOT.thread_bot();
    unsafe {
        INDEXED_URLS_NB = get_rows_number();
    }
    if !cargs.contains("--no-queue-recover".to_string()) {
        let rurls = read_lines("./runtime/queue").unwrap_or(vec![]);

        println!("[QUEUE] Recovered {} URLs", rurls.len());
        QUEUE_BOT.queue_url(rurls);
    }

    let _ = rocket::custom(pg_figment)
        .attach(Pg::init())
        .attach(CORS)
        .mount("/", routes![
            index_websites, 
            search_query, 
            search_default_ui,
            index_websites_from_robots,
            console_ui
        ])
        .mount("/debug", routes![toggle_queue_bot])
        .mount("/static", FileServer::from(relative!("/static")))
        .mount("/assets", FileServer::from(relative!("/static/assets")))
        .mount("/api", routes![
            get_index_sys_status,
            get_domain_ownership_key,
            check_domain_ownership
        ])
        .launch()
        .await;

    write_lines("./runtime/queue", QUEUE_BOT.get_remaining_urls())
        .expect("werr");
}
