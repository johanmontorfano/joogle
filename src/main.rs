mod indexing;
mod sanitize;
mod searching;
mod templates;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
#[macro_use] extern crate rocket;

use lazy_static::lazy_static;
use indexing::index_url;
use maud::Markup;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rocket::{fs::{FileServer, relative}, serde::json::Json};
use searching::feeling_lucky;
use templates::{indexing::indexing_page, search::search_result_page};

lazy_static! {
    static ref DB_POOL: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file("index_db.db");
        Pool::new(manager).unwrap()
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
    let mut proc_results: Vec<(String, String)> = vec![];

    for url in url_list.iter() {
        if let Err(error) = index_url(url.into()).await {
            proc_results.push((url.clone(), error.to_string()));
        } else {
            proc_results.push((url.clone(), "OK".into()));
        }
    }
    indexing_page(proc_results)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index_websites, search_query, search_default_ui])
        .mount("/static", FileServer::from(relative!("/static")))
}
