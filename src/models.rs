use std::io::Write;

use diesel::{deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, pg::{Pg, PgValue}, serialize::{self, IsNull, Output, ToSql}};
use ipnet::IpNet;
use rocket_db_pools::diesel::prelude::*;
use uuid::Uuid;
use serde_json::Value;
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::schemas::{_auth, _public};

#[derive(Debug, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = _auth::sql_types::AalLevel)]
pub enum AalLevel {
    Aal1,
    Aal2
}

impl ToSql<_auth::sql_types::AalLevel, Pg> for AalLevel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            AalLevel::Aal1 => out.write_all(b"aal1")?,
            AalLevel::Aal2 => out.write_all(b"aal2")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<_auth::sql_types::AalLevel, Pg> for AalLevel {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"aal1" => Ok(AalLevel::Aal1),
            b"aal2" => Ok(AalLevel::Aal2),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
#[derive(Insertable)]
#[diesel(table_name = _public::domains)]
pub struct AddDomainOwnership {
    pub domain: String,
    pub owned_by: Uuid,
}

#[derive(Selectable, Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = _auth::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUsers {
    pub instance_id: Option<Uuid>,
    pub id: Uuid,
    pub aud: Option<String>,
    pub role: Option<String>,
    pub email: Option<String>,
    pub encrypted_password: Option<String>,
    pub email_confirmed_at: Option<DateTime<Utc>>,
    pub invited_at: Option<DateTime<Utc>>,
    pub confirmation_token: Option<String>,
    pub confirmation_sent_at: Option<DateTime<Utc>>,
    pub recovery_token: Option<String>,
    pub recovery_sent_at: Option<DateTime<Utc>>,
    pub email_change_token_new: Option<String>,
    pub email_change: Option<String>,
    pub email_change_sent_at: Option<DateTime<Utc>>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub raw_app_meta_data: Option<Value>,
    pub raw_user_meta_data: Option<Value>,
    pub is_super_admin: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub phone: Option<String>,
    pub phone_confirmed_at: Option<DateTime<Utc>>,
    pub phone_change: Option<String>,
    pub phone_change_token: Option<String>,
    pub phone_change_sent_at: Option<DateTime<Utc>>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub email_change_token_current: Option<String>,
    pub email_change_confirm_status: Option<i16>,
    pub banned_until: Option<DateTime<Utc>>,
    pub reauthentication_token: Option<String>,
    pub reauthentication_sent_at: Option<DateTime<Utc>>,
    pub is_sso_user: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_anonymous: bool,
}

#[derive(Selectable, Queryable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = _auth::sessions)]
#[diesel(belongs_to(AuthUsers, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub factor_id: Option<Uuid>,
    pub aal: Option<AalLevel>,
    pub not_after: Option<DateTime<Utc>>,
    pub refreshed_at: Option<NaiveDateTime>,
    pub user_agent: Option<String>,
    pub ip: Option<IpNet>,
    pub tag: Option<String>,
}
