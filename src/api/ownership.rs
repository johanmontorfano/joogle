use rocket::response::status;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, Resolver, TokioAsyncResolver};
use url::Url;

fn create_dns_record_for_domain_reg(domain: String, uid: String) -> String {
    format!("joogleown:{domain}>{uid}")
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
pub fn get_domain_ownership_key(domain: String, uid: String) -> String {
    let url = Url::parse(&domain);
    let url = url.unwrap();

    let domain: String = url.domain().unwrap().into();

    create_dns_record_for_domain_reg(domain, uid)
}

/// As described in the above route. Domain ownership is verified through DNS
/// verification. Here, we verify the DNS entries of a given domain and
/// validate or no the ownership of the said domain.
#[get("/domain/check_dns_record?<domain>&<uid>")]
pub async fn check_domain_ownership(domain: String, uid: String) -> String {
    let url = Url::parse(&domain);
    let url = url.unwrap();

    let domain: String = url.domain().unwrap().into();

    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
    let res = resolver.txt_lookup(domain.clone()).await.unwrap();

    let ownership_rec = create_dns_record_for_domain_reg(domain, uid);

    for txt in res.iter() {
        println!("{} == {}", txt, ownership_rec);
        if txt.to_string() == ownership_rec {
            return "TRUE".to_string();
        }
    }
    "FALSE".to_string()
}
