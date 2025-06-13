use serde_derive::{Deserialize, Serialize};
use url::Url;
use crate::indexer::localization::Localization;
use crate::DB_POOL;
use crate::sanitize::sql_escape_ap;

/// Refers to an indexed page
/// NOTE: When moving from SQL to Diesel + Pg, please, PLEASE refer to pages
/// instead of sites.
#[derive(Clone, Serialize, Deserialize)]
pub struct SiteRecord {
    pub url: String,
    pub domain: String,
    pub title: String,
    pub description: String,
    pub ttr: f64,
    pub loc: String
}

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
    let url = sql_escape_ap(url);
    let title = sql_escape_ap(title);
    let description = sql_escape_ap(description);

    conn.execute(&format!("DELETE FROM sites WHERE url = '{url}'"), [])?;
    conn.execute(&format!("
            INSERT INTO sites (url, domain, title, description, ttr, loc) 
            VALUES ('{url}', '{domain}', '{title}', '{description}', 0.0, 'en')
    "), [])?;
    Ok(())
}

/// Updates the Type-Token Ratio of a site.
pub fn update_site_ttr(
    url: &String, ttr: f64
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let url = sql_escape_ap(url.into());

    conn.execute(
        &format!("UPDATE sites SET ttr = {ttr} WHERE url = '{url}'"), []
    )?;
    Ok(())
}

/// Updates the Localization of a site.
pub fn update_site_loc(
    url: &String, loc: Localization
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let url = sql_escape_ap(url.into());

    conn.execute(
        &format!("UPDATE sites SET loc = '{}' WHERE url = '{url}'", loc.0), []
    )?;
    Ok(())
}

pub fn get_all_sites_records_of_a_domain(
    domain: String
) -> Result<Vec<SiteRecord>, Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    
    let mut select = conn.prepare(&format!("
        SELECT *
        FROM sites
        WHERE domain LIKE '%{domain}'
    ")).unwrap();

    let results = select.query_map([], |row| Ok((
        row.get::<usize, String>(0).unwrap(),
        row.get::<usize, String>(1).unwrap(),
        row.get::<usize, String>(2).unwrap(),
        row.get::<usize, String>(3).unwrap(),
        row.get::<usize, f64>(4).unwrap(),
        row.get::<usize, String>(5).unwrap()
    ))).unwrap();

    let mut out: Vec<SiteRecord> = vec![];

    results.for_each(|r| {
        let r = r.unwrap();

        out.push(SiteRecord {
            url: r.0,
            domain: r.1,
            title: r.2,
            description: r.3,
            ttr: r.4,
            loc: r.5
        });
    });
    Ok(out)
}
