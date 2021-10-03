use crate::AppConfig;
use chrono::Duration;
use hmac::{Hmac, NewMac};
use jwt::{Claims, RegisteredClaims, SignWithKey, VerifyWithKey};
use rocket::http::hyper::header::AUTHORIZATION;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use sha2::Sha256;
use std::fmt::Debug;

#[derive(Clone)]
pub struct TokenService {
    pub secret_key: String,
}

impl TokenService {
    pub fn signing(&self, sub: &str) -> String {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(self.secret_key.as_bytes()).expect("failed to parse secret_key");
        let mut claims: Claims = Claims::new(RegisteredClaims::default());
        claims.registered.issuer = Some("".to_string()); // .insert("ediary_api".to_string());

        claims.registered.issued_at = Some(chrono::Utc::now().timestamp() as u64);

        claims.registered.subject = Some(String::from(sub));

        let hour = chrono::Utc::now() + Duration::hours(1);
        claims.registered.expiration = Some(hour.timestamp() as u64);
        claims.sign_with_key(&key).expect("failed to signing")
    }

    pub fn verify(&self, token: &str) -> bool {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(self.secret_key.as_bytes()).expect("failed to parse secret_key");
        let claims: Claims = token
            .verify_with_key(&key)
            .expect("failed to verify claims");
        let issuer_is_valid = match claims.registered.issuer {
            Some(value) => value == "ediary_api",
            None => false,
        };
        let is_not_expired = match claims.registered.expiration {
            Some(value) => chrono::Utc::now().timestamp() as u64 > value,
            None => false,
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

        Outcome::Success(TokenService {
            secret_key: app_config.token_secret_key.clone(),
        })
    }
}

pub struct Token<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let app_config = request
            .rocket()
            .state::<AppConfig>()
            .expect("token service is not defined");
        let token_service = TokenService {
            secret_key: app_config.token_secret_key.clone(),
        };

        match request.headers().get_one(AUTHORIZATION.as_str()) {
            Some(header_value) if token_service.verify(header_value) => {
                Outcome::Success(Token(header_value))
            }
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
            _ => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
        }
    }
}
