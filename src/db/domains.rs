use std::{collections::HashMap, str::FromStr};
use crate::{models::{AddDomainOwnership, DomainOwnershipRecord}, schemas::_public::domains};
use rocket_db_pools::Connection;
use uuid::Uuid;
use crate::{sanitize::{sql_encode_uas, sql_escape_ap}, Pg, DB_POOL};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};

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
pub async fn update_domain_ownership_record(
    mut pg: Connection<Pg>,
    domain: String,
    owned_by: String
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_POOL.clone().get().unwrap();
    let domain = sql_escape_ap(domain);

    let new_ownership = AddDomainOwnership {
        domain: domain.clone(),
        owned_by: Uuid::from_str(&owned_by)?
    };

    conn.execute(&format!("
        UPDATE domains
        SET owned_by_uid = '{owned_by}'
        WHERE domain LIKE '%{domain}'
    "), [])?;

    async {
        use rocket_db_pools::diesel::prelude::RunQueryDsl;

        let _ = diesel::insert_into(domains::table)
            .values(&new_ownership)
            .execute(&mut pg)
            .await;
    }.await;

    Ok(())
}

pub async fn get_domain_ownership_record(
    mut pg: Connection<Pg>,
    domain: String
) -> Result<DomainOwnershipRecord, ()> {
    use rocket_db_pools::diesel::prelude::RunQueryDsl;

    let domains_list = domains::table
        .filter(domains::domain.eq(domain))
        .select(DomainOwnershipRecord::as_select())
        .load::<DomainOwnershipRecord>(&mut pg)
        .await;

    if domains_list.is_err() {
        return Err(());
    }

    let domains_list = domains_list.unwrap();
    let domains_list = domains_list.iter().collect::<Vec<_>>();

    if domains_list.len() != 1 {
        return Err(());
    }

    let out = domains_list.get(0).unwrap();

    // TODO: wtf do I have to .clone().clone() ??????
    Ok((*out).clone())
}
