use std::fmt::Debug;

use hmac::{Hmac, NewMac};
use jwt::{Claims, Error, RegisteredClaims, SignWithKey, VerifyWithKey};
use r2d2::PooledConnection;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use sha2::Sha256;

use crate::models::errors::AuthError;
use crate::models::role::Role;
use crate::repository::role::get_roles_by_username;
use crate::{AppConfig, MainDb};
use redis::Commands;
use rocket::http::hyper::header::AUTHORIZATION;

pub struct TokenService {
    pub config: AppConfig,
    pub redis_conn: PooledConnection<redis::Client>,
}

impl TokenService {
    const BEARER: &'static str = "Bearer";

    pub fn signing(&self, sub: &str, exp: u64) -> Result<String, Error> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.config.token_secret_key.as_bytes())
            .expect("failed to parse secret_key");
        let mut claims: Claims = Claims::new(RegisteredClaims::default());
        claims.registered.issuer = Some("ediary_api".to_string()); // .insert("ediary_api".to_string());

        claims.registered.issued_at = Some(chrono::Utc::now().timestamp() as u64);

        claims.registered.subject = Some(String::from(sub));

        claims.registered.expiration = Some(exp);
        claims.sign_with_key(&key)
    }

    pub fn claims(&self, token: &str) -> Result<Claims, Error> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.config.token_secret_key.as_bytes())
            .expect("failed to parse secret_key");
        match token.verify_with_key(&key) {
            Ok(claims) => Ok(claims),
            Err(_) => Err(Error::InvalidSignature),
        }
    }

    pub fn verify(&mut self, token: &str) -> bool {
        if !token.starts_with(TokenService::BEARER) {
            return false;
        }
        let token: String = String::from(&token[TokenService::BEARER.len() + 1..]);

        let claims = match self.claims(token.clone().as_str()) {
            Ok(claims) => claims,
            Err(e) => {
                println!("{:?}", e);
                return false;
            }
        };
        let token_blacklist: Vec<String> =
            self.redis_conn.lrange("token_blacklist", 0, -1).unwrap();
        if token_blacklist.iter().any(|t| t == &token) {
            return false;
        }

        let issuer_is_valid = match claims.registered.issuer {
            Some(value) => value == "ediary_api",
            None => false,
        };
        let is_not_expired = match claims.registered.expiration {
            Some(value) => value > chrono::Utc::now().timestamp() as u64,
            None => {
                self.redis_conn
                    .rpush::<&str, String, i32>("token_blacklist", token);
                false
            }
        };

        issuer_is_valid && is_not_expired
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenService {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let app_config = request
            .rocket()
            .state::<AppConfig>()
            .expect("token service is not defined");
        let pool = request
            .rocket()
            .state::<r2d2::Pool<redis::Client>>()
            .unwrap();
        let conn = pool.get().unwrap();

        Outcome::Success(TokenService {
            config: app_config.clone(),
            redis_conn: conn,
        })
    }
}

#[derive(Debug)]
pub struct Token<'r> {
    pub token: &'r str,
    pub roles: Vec<Role>,
}

impl<'r> Token<'r> {
    pub fn has_role(&self, name: &str) -> bool {
        self.roles.iter().any(|r| r.name == name)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let app_config = request
            .rocket()
            .state::<AppConfig>()
            .expect("AppConfig is not defined");
        let pool = request
            .rocket()
            .state::<r2d2::Pool<redis::Client>>()
            .unwrap();
        let pg_conn = request.guard::<MainDb>().await.unwrap();
        let conn = pool.get().unwrap();

        let mut token_service = TokenService {
            config: app_config.clone(),
            redis_conn: conn,
        };

        match request.headers().get_one(AUTHORIZATION.as_str()) {
            Some(header_value) if token_service.verify(header_value) => {
                let token_str = &header_value[TokenService::BEARER.len() + 1..];
                let claims = token_service.claims(token_str).unwrap();
                return if let Some(username) = claims.registered.subject {
                    if app_config.admin.as_ref().unwrap() == &username {
                        return Outcome::Success(Token {
                            token: token_str,
                            roles: vec![Role {
                                id: -1,
                                name: "ADMIN".to_string(),
                                description: None,
                            }],
                        });
                    }
                    let roles: Vec<Role> = pg_conn
                        .run(move |c| get_roles_by_username(c, &username))
                        .await;
                    Outcome::Success(Token {
                        token: &header_value[TokenService::BEARER.len() + 1..],
                        roles,
                    })
                } else {
                    Outcome::Failure((Status::Unauthorized, AuthError::InvalidToken))
                };
            }
            None => Outcome::Failure((Status::Unauthorized, AuthError::MissingAuthHeader)),
            _ => Outcome::Failure((Status::Unauthorized, AuthError::InvalidToken)),
        }
    }
}
