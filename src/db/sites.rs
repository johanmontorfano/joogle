use crate::DB_POOL;

/// Escape ' to avoid issues on SQL insertion.
fn sql_escape(item: String) -> String {
    item.replace("'", "''")
}

/// Initialize this table if it does not exists on the database.
pub fn init_table() -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();

    conn.execute("
        CREATE TABLE IF NOT EXISTS sites (
            url TEXT PRIMARY KEY,
            title TEXT,
            description TEXT,
            ttr REAL
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
    let url = sql_escape(url);
    let title = sql_escape(title);
    let description = sql_escape(description);

    conn.execute(&format!("DELETE FROM sites WHERE url = '{url}'"), [])?;
    conn.execute(&format!("
        INSERT INTO sites (url, title, description, ttr) VALUES (
            '{url}', '{title}', '{description}', 0.0
        );
    "), [])?;
    Ok(())
}

/// Updates the Type-Token Ratio of a site.
pub fn update_site_ttr(
    url: String, ttr: f64
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let url = sql_escape(url);

    conn.execute(
        &format!("UPDATE sites SET ttr = {ttr} WHERE url = '{url}'"), []
    )?;
    Ok(())
}
