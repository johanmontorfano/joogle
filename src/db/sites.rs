use url::Url;

use crate::DB_POOL;
use crate::sanitize::sql_escape_ap;

/// Initialize this table if it does not exists on the database.
pub fn init_table() -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();

    conn.execute("
        CREATE TABLE IF NOT EXISTS sites (
            url TEXT PRIMARY KEY,
            domain TEXT,
            title TEXT,
            description TEXT,
            ttr REAL,
            CONSTRAINT domain FOREIGN KEY (domain) REFERENCES domains(domain)
        )
    ", [])?;
    Ok(())
}

/// Create a new record of an indexed URL, multiple URLs of the same websites 
/// will always have a unique record here.
/// WARN: If this function is called with an URL already present in the 
/// database, the previous record and all the data linked to it will be deleted.
pub fn new_url_record(
    url: String, 
    title: String, 
    description: String
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let url_obj = Url::parse(&url)?;
    let domain = sql_escape_ap(url_obj.domain().unwrap().to_string());
    let url = sql_escape_ap(url);
    let title = sql_escape_ap(title);
    let description = sql_escape_ap(description);

    conn.execute(&format!("DELETE FROM sites WHERE url = '{url}'"), [])?;
    conn.execute(&format!("
            INSERT INTO sites (url, domain, title, description, ttr) 
            VALUES ('{url}', '{domain}', '{title}', '{description}', 0.0)
    "), [])?;
    Ok(())
}

/// Updates the Type-Token Ratio of a site.
pub fn update_site_ttr(
    url: String, ttr: f64
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let url = sql_escape_ap(url);

    conn.execute(
        &format!("UPDATE sites SET ttr = {ttr} WHERE url = '{url}'"), []
    )?;
    Ok(())
}
