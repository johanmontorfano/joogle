use crate::DB_POOL;

/// Initialize this table if it does not exists on the database.
pub fn init_table() -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();

    conn.execute("
        CREATE TABLE IF NOT EXISTS sites (
            url text PRIMARY KEY,
            title text,
            description text
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

    conn.execute_batch(&format!("
        BEGIN;
        DELETE FROM sites WHERE url = '{url}';
        INSERT INTO sites (url, title, description) VALUES (
            '{url}', '{title}', '{description}'
        );
        COMMIT;
    "))?;
    Ok(())
}
