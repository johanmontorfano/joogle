use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};
use trust_dns_resolver::TokioAsyncResolver;
use rocket_db_pools::Connection;
use url::{ParseError, Url};
use crate::{db::{domains::update_domain_ownership_record, jwt_auth::AuthFromJWT}, Pg, QUEUE_BOT};

#[derive(Serialize, Deserialize)]
pub struct ResOwnershipVerification {
    queue_position: usize,
    ownership_verified: bool
}

#[derive(Serialize, Deserialize)]
pub struct ResOwnershipVerificationTXTValue {
    for_domain: String,
    for_user: String,
    txt_record_content: String
}

/// TODO: Search analytics such as search results appearance and clicks must
/// be added here.
#[derive(Serialize, Deserialize)]
struct SiteAnalyticsData {
    url: String,
    title: String
}

/// This represents the data of a single domain.
#[derive(Serialize, Deserialize)]
pub struct ResAnalyticsData {
    domain: String,
    owned_by: String,
    created_at: DateTime<Utc>,
    last_indexed_at: DateTime<Utc>,
    sites: Vec<SiteAnalyticsData>
}

fn create_dns_record_for_domain_reg(domain: String, uid: String) -> String {
    format!("joogleown:{domain}>{uid}")
}

/// Since `url::Url` may provoke the `RelativeUrlWithoutBase` error based on
/// user input, we want to overcome this potential issue by automatically
/// adding a base to the URL when none are provided.
fn extract_domain_from_str(from: String) -> Result<String, ParseError> {
    let url = Url::parse(&from);

    if let Err(err) = url {
        if err == ParseError::RelativeUrlWithoutBase {
            // Hoping it will cover most cases.
            return extract_domain_from_str(format!("http://{from}"));
        }
    }

    return Ok(url.unwrap().domain().unwrap().to_string()); 
}

/// *First reference to the concept of domain ownership.*
/// Indexing a domain can be done by any actors. However, we allow users to
/// own their domains by modifying DNS registries. It can be useful later to
/// have domain ownership to implement insights, or indexing infos. And also
/// to prevent page indexing from anyone (it will make possible to enforce a
/// new rule: allowing non-owned domain indexing by the robots only)
///
/// WARN: The documentation below is relevant only while the whole project
/// works on two databases: SQLite (cargo app) and Postgres Supabase 
/// (cargo app + SPAS)
/// 
/// To properly identify domain owners, we create a challenge that has to be
/// added to the DNS as a TXT key. This challenge will contain the user UID
/// only.
///
/// INFO: Whatever the level of complexity of the challenge is, it always have
/// to be created by the cargo app.
///
/// When the cargo app verifies DNS records of the domain upon user action, it
/// will validate on the postgres AND sqlite database the ownership of the
/// domain, and indexing will start.
#[get("/domain/get_ownership?<domain>&<uid>")]
pub fn get_domain_ownership_key(
    domain: String,
    uid: String
) -> Json<ResOwnershipVerificationTXTValue> {
    let domain = extract_domain_from_str(domain).unwrap();

    Json(ResOwnershipVerificationTXTValue {
        for_domain: domain.clone(),
        for_user: uid.clone(),
        txt_record_content: create_dns_record_for_domain_reg(domain, uid)
    })
}

/// As described in the above route. Domain ownership is verified through DNS
/// verification. Here, we verify the DNS entries of a given domain and
/// validate or no the ownership of the said domain.
#[get("/domain/check_dns_record?<domain>&<uid>")]
pub async fn check_domain_ownership(
    pg: Connection<Pg>,
    domain: String,
    uid: String
) -> Json<ResOwnershipVerification> {
    let domain = extract_domain_from_str(domain).unwrap();

    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
    let res = resolver.txt_lookup(domain.clone()).await.unwrap();

    let ownership_rec = create_dns_record_for_domain_reg(
        domain.clone(), uid.clone());

    for txt in res.iter() {
        if txt.to_string() == ownership_rec {
            update_domain_ownership_record(pg, domain.clone(), uid)
                .await.unwrap();
            QUEUE_BOT.queue_url(vec![domain]);

            let queue_position = QUEUE_BOT.get_remaining_urls().len();

            return Json(ResOwnershipVerification {
                queue_position,
                ownership_verified: true
            });
        }
    }
    Json(ResOwnershipVerification {
        queue_position: 0,
        ownership_verified: false
    })
}

/// Users can retrieve analytics to the domains they own. Analytics range from
/// search analytics (search apparitions, clicks, ...) and indexing analytics.
/// To verify ownership of a domain, the client has to provide a JWT that will
/// be verified against Postgres content (where the data is stored)
#[get("/domain/get_analytics?<domain>")]
pub async fn get_domain_analytics(
    pg: Connection<Pg>,
    auth: AuthFromJWT,
    domain: String
) -> Result<Json<ResAnalyticsData>, ()> {
    todo!("Add analytics data retrieving first");
}
