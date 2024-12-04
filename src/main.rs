#![feature(thread_sleep_until)]
#[macro_use]
extern crate rocket;
extern crate r2d2;
extern crate r2d2_postgres;

mod db;
mod error;
mod macros;
mod indexer;
mod sanitize;
mod searching;
mod templates;
mod data_pool;
#[cfg(feature = "debug")]
mod debug;

use db::local::{read_lines, write_lines};
use lazy_static::lazy_static;
use maud::Markup;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use rocket::{fs::{FileServer, relative}, serde::json::Json};
use searching::feeling_lucky;
use templates::{indexing::indexing_page, search::search_result_page};
use indexer::url::QueueBot;
use indexer::sitemaps::SitemapBot;

lazy_static! {
    static ref DB_POOL: Pool<PostgresConnectionManager<NoTls>> = {
        let manager = PostgresConnectionManager::new(
            "host=localhost user=joogle dbname=joogle_db password=devpassword port=5432".parse().unwrap(),
            NoTls
        );
        Pool::new(manager).unwrap()
    };
    static ref SITEMAP_BOT: indexer::sitemaps::SitemapBot = {
        SitemapBot::init()
    };
    static ref QUEUE_BOT: indexer::url::QueueBot = {
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

#[post("/index/urls", data = "<url_list>")]
fn index_websites(url_list: Json<Vec<String>>) -> Markup {
    QUEUE_BOT.queue_url(url_list.0);
    indexing_page()
}

/// It's important to submit a domain to this route as `RobotsDefinition` will
/// not be able in every scenario to use a URL properly and is intended to use
/// a domain name.
#[cfg(all(feature = "sitemaps_protocol", feature = "robots_protocol"))]
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
    let cores: usize = std::thread::available_parallelism().unwrap().into();
    let recovered_queue = read_lines("./runtime/queue").unwrap_or(vec![]);

    db::sites::init_table().expect("Failed to init 'sites' table.");
    db::domains::init_table().expect("Failed to init 'domains' table.");
    QUEUE_BOT.queue_url(recovered_queue);
    (0..cores.min(2)).for_each(|_| QUEUE_BOT.thread_bot() );
    SITEMAP_BOT.thread_bot();

    let mut builder = rocket::build()
        .mount("/", routes![index_websites, search_query, search_default_ui])
        .mount("/static", FileServer::from(relative!("/static")));
    ifcfg!("debug", { 
        builder = builder.mount("/debug", routes![
            debug::routes::toggle_queue_bot
        ]);
    });
    ifcfg!("sitemaps_protocol", {
        ifcfg!("robots_protocol", {
            builder = builder.mount("/", routes![index_websites_from_robots]);
        });
    });

    let _ = builder.launch().await;
    write_lines("./runtime/queue", QUEUE_BOT.get_remaining_urls())
        .expect("werr");
}
