use std::{collections::HashMap, str::FromStr, time::{SystemTime, UNIX_EPOCH}};
use url::Url;
use crate::{db::domains, sanitize::{sql_decode_uas, sql_escape_ap}, DB_POOL};

/// We need a custom robots.txt parser as none exists for Rust. The convention
/// for robots.txt is fairly easy so it's should not be any implementation
/// issues.
/// Sitemaps declared in the robots file are not saved on the database.
pub struct RobotsDefinition {
    pub domain: String,
    pub uas_disallow: HashMap<String, Vec<String>>,
    pub uas_allow: HashMap<String, Vec<String>>,
    pub sitemaps: Vec<String>
}

impl RobotsDefinition {
    pub async fn from_domain(
        domain: String
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut url = Url::from_str(&domain)?;
        url.set_scheme("https").unwrap();
        url.set_path("robots.txt");

        let content = surf::get(&url.to_string()).recv_string().await?;
        let mut curr_ua = String::new();
        let mut uas_allow: HashMap<String, Vec<String>> = HashMap::new();
        let mut uas_disallow: HashMap<String, Vec<String>> = HashMap::new();
        let mut sitemaps = vec![];

        content
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with("#"))
            .for_each(|line| {
                let (directive, value) = line.split_once(":").unwrap();
                let directive = directive.to_lowercase();
                let value = value.replace(" ", "");

                match directive.as_str() {
                    "user-agent" => { curr_ua = value; }
                    "allow" => {
                        if let Some(ua_ref) = uas_allow.get_mut(&curr_ua) {
                            ua_ref.push(value);
                        } else {
                            uas_allow.insert(curr_ua.clone(), vec![value]);
                        }
                    }
                    "disallow" => {
                        if let Some(ua_ref) = uas_disallow.get_mut(&curr_ua) {
                            ua_ref.push(value);
                        } else {
                            uas_disallow.insert(curr_ua.clone(), vec![value]);
                        }
                    }
                    "sitemap" => { sitemaps.push(value); }
                    _ => ()
                }
            });

        Ok(Self { domain, uas_disallow, uas_allow, sitemaps })
    }

    /// TODO: Improve this function's efficiency.
    /// WARN: Using this function to recover a domain's data will not recover
    /// sitemaps.
    pub fn from_db(domain: String) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = DB_POOL.clone().get().unwrap();
        let domain = sql_escape_ap(domain);
        let mut select_stmt = conn.prepare(&format!("
            SELECT uas_allow, uas_disallow
            FROM domains
            WHERE domain = '{domain}'
        "))?;
        let domain_iter = select_stmt
            .query_map([], |row| Ok((
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, String>(1).unwrap()
            )))?
            .map(|d| d.unwrap())
            .collect::<Vec<(String, String)>>();
        let (uas_allow, uas_disallow) = domain_iter.get(0).unwrap();

        Ok(Self {
            domain,
            uas_allow: sql_decode_uas(uas_allow.to_string()),
            uas_disallow: sql_decode_uas(uas_disallow.to_string()),
            sitemaps: vec![]
        })
    }

    pub fn db_save(&self) -> Result<(), Box<dyn std::error::Error>> {
        domains::create_row(
            self.domain.clone(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(), 
            self.uas_allow.clone(), 
            self.uas_disallow.clone()
        )?;
        Ok(())
    }
}
