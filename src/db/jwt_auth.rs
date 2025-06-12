use std::{env, str::FromStr};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::{http::Status, request::{FromRequest, Outcome}, Request};
use rocket_db_pools::{diesel::prelude::RunQueryDsl, Connection};
use serde_derive::{Serialize, Deserialize};
use crate::{models::{AuthSession, AuthUsers}, Pg};
use crate::schemas::_auth::{sessions, users};
use uuid::Uuid;

/// WARN: *_id are Uuid but cannot be serialized, hence provided as String.
#[derive(Serialize, Deserialize, Debug)]
struct AuthClaims {
    pub email: String,
    pub user_id: String,
    pub session_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthFromJWT {
    from_claims: AuthClaims,
    verified: bool
}

/// This function will authenticate a user from a JWT and the content of the
/// Postgres database. We expect the JWT to have the following content:
/// { email, user_id, session_id }
///
/// To verify the authenticity and validity of the JWT and the client, we will
/// verify the user AND session exists. And verify if the client has the same
/// IP as the session
///
/// This guard requires the `Authorization` header to be provided
///
/// Every JWT has to be signed with the `JWT_SECRET` environment variable
/// INFO: This is a request guard, it should be directly implemented in a req
/// WARN: The current implementation heavily relies on Supabase's `auth` table
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthFromJWT {
    type Error = String;

    async fn from_request(
        req: &'r Request<'_>
    ) -> Outcome<Self, Self::Error> {
        let mut pg = Connection::<Pg>::from_request(req).await.unwrap();
        let jwt = req.headers().get_one("Authorization");

        if jwt.is_none() {
            return Outcome::Error((
                Status::BadRequest,
                "Authorization header is missing".to_string()
            ));
        }

        let jwt = jwt.unwrap();

        // We retrieve the JWT data and verify its validity with the env
        // containing the secret.

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET missing");
        let dec_key = DecodingKey::from_secret(secret.as_bytes());
        let validator = Validation::new(Algorithm::HS256);
        let claims = decode::<AuthClaims>(&jwt, &dec_key, &validator);

        if let Err(err) = claims {
            println!("AuthResult Guard: {}", err);

            return Outcome::Error((
                Status::InternalServerError,
                "JWT decoding failed: invalid JWT".to_string()
            ));
        }

        let auth_claims = claims.unwrap().claims;
        let uuid = Uuid::from_str(&auth_claims.user_id).unwrap();

        // user and session are joined and retrieved for this user

        let user_and_session = sessions::table
            .inner_join(users::table)
            .filter(users::id.eq(uuid))
            .select((AuthSession::as_select(), AuthUsers::as_select()))
            .load::<(AuthSession, AuthUsers)>(&mut pg)
            .await;


        if let Err(err) = user_and_session {
            println!("AuthResult Guard: {}", err);

            return Outcome::Error((
                Status::InternalServerError,
                "JWT decoding failed: cannot retrieve data".to_string()
            ));
        }

        let user_and_session = user_and_session.unwrap();
        let verified_results = user_and_session.iter()
            .filter(|row| {
                row.1.email.clone().unwrap() == auth_claims.email &&
                    row.0.user_id.to_string() == auth_claims.user_id &&
                    row.0.id.to_string() == auth_claims.session_id
            })
            .collect::<Vec<_>>();

        if verified_results.len() == 1 {
            return Outcome::Success(AuthFromJWT {
                from_claims: auth_claims,
                verified: true
            });
        }

        Outcome::Error((
            Status::InternalServerError,
            "JWT decoding failed: cannot authenticate".to_string()
        ))
    }
}
