use url::Url;

use crate::indexer::localization::Localization;
use crate::DB_POOL;
use crate::sanitize::sql_escape_ap;

/// Initialize this table if it does not exists on the database.
pub fn init_table() -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS sites (
        url TEXT PRIMARY KEY,
        domain TEXT,
        title TEXT,
        description TEXT,
        ttr REAL,
        loc TEXT,
        CONSTRAINT domain FOREIGN KEY (domain) REFERENCES domains(domain)
    )", [])?;
    Ok(())
}

/// Returns the number of rows of the `sites` table. 
/// INFO: For performance reasons, PLEASE DO NOT CALL THIS FUNCTION TOO OFTEN
pub fn get_rows_number() -> isize {
    let conn = DB_POOL.clone().get().unwrap();
    let mut query = conn.prepare("SELECT COUNT(1) FROM sites").unwrap();
    
    if let Ok(mut rows) = query.query([]) {
        let row_0 = rows.next().unwrap();
        
        return row_0.unwrap().get::<usize, isize>(0).unwrap();
    }
    return -1;
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
    let title = sql_escape_ap(title);
    let description = sql_escape_ap(description);

    conn.execute("DELETE FROM sites WHERE url = ?1", [url.clone()])?;
    conn.execute("
        INSERT INTO sites (url, domain, title, description, ttr, loc) 
        VALUES (?1, ?2, ?3, ?4, 0.0, 'en')
    ", [url, domain, title, description])?;
    Ok(())
}

/// Updates the Type-Token Ratio of a site.
pub fn update_site_ttr(
    url: &String, ttr: f64
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();

    conn.execute(
        "UPDATE sites SET ttr = ?1 WHERE url = ?2",
        params![ttr, url]
    )?;
    Ok(())
}

/// Updates the Localization of a site.
pub fn update_site_loc(
    url: &String, loc: Localization
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();

    conn.execute(
        "UPDATE sites SET loc = ?1 WHERE url = ?2",
        params![loc.0, url]
    )?;
    Ok(())
}
