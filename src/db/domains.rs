use std::collections::HashMap;
use crate::{sanitize::{sql_encode_uas, sql_escape_ap}, DB_POOL};

// WARN: IMPORTANT NOTICE FOR THIS TABLE
// The `uas_allow` and `uas_disallow` columns are stringified hashmaps, those
// must be encoded and decoded using the `sql_encode_uas` and `sql_decode_uas`
// functions from the `sanitize` module.

/// Initalizes the table if it doesn't exists already.
pub fn init_table() -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();

    conn.execute("
        CREATE TABLE IF NOT EXISTS domains (
            domain TEXT PRIMARY KEY,
            last_robots_txt_visit INTEGER,
            last_ownership_check INTEGER,
            uas_allow TEXT,
            uas_disallow TEXT,
            owned_by_uid TEXT
        )
    ", [])?;
    Ok(())
}

/// Saves data of a domain to the database.
/// WARN: If a row for this domain already exists, every value get updated.
pub fn create_row(
    domain: String,
    last_robots_txt_visit: u128,
    last_ownership_check: u128,
    uas_allow: HashMap<String, Vec<String>>,
    uas_disallow: HashMap<String, Vec<String>>,
    owned_by_uid: String
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let domain = sql_escape_ap(domain);
    let uas_allow = sql_escape_ap(sql_encode_uas(uas_allow));
    let uas_disallow = sql_escape_ap(sql_encode_uas(uas_disallow));

    conn.execute(&format!("
        INSERT OR REPLACE INTO domains (
            domain,
            last_robots_txt_visit,
            last_ownership_check,
            uas_allow,
            uas_disallow,
            owned_by_uid
        )
        VALUES (
            '{domain}',
            {last_robots_txt_visit},
            {last_ownership_check},
            '{uas_allow}',
            '{uas_disallow}',
            '{owned_by_uid}'
        )
    "), [])?;
    Ok(())
}

/// Save data of a domain to the database iff a row for this domain doesn't 
/// already exist.
pub fn create_row_iff_empty(
    domain: String,
    last_robots_txt_visit: u128,
    last_ownership_check: u128,
    uas_allow: HashMap<String, Vec<String>>,
    uas_disallow: HashMap<String, Vec<String>>,
    owned_by_uid: String
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let domain = sql_escape_ap(domain);
    let uas_allow = sql_escape_ap(sql_encode_uas(uas_allow));
    let uas_disallow = sql_escape_ap(sql_encode_uas(uas_disallow));

    conn.execute(&format!("
        INSERT OR REPLACE INTO domains (
            domain,
            last_robots_txt_visit,
            last_ownership_check,
            uas_allow,
            uas_disallow,
            owned_by_uid
        )
        VALUES (
            '{domain}',
            {last_robots_txt_visit},
            {last_ownership_check},
            '{uas_allow}',
            '{uas_disallow}',
            '{owned_by_uid}'
        )
    "), [])?;
    Ok(())
}

/// Update ownership data for a domain. If the domain does not exist, the
/// operation is aborted. It will also modify the content of the postgres
/// database.
/// TODO: Update `last_ownership_check`
pub fn update_domain_ownership_record(
    domain: String,
    owned_by: String
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let domain = sql_escape_ap(domain);

    conn.execute(&format!("
        UPDATE domains
        SET owned_by_uid = '{owned_by}'
        WHERE domain = '{domain}'
    "), [])?;
    Ok(())
}
