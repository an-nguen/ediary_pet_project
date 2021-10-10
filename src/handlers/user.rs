use actix_web::{web, HttpResponse, ResponseError};

use crate::db::datasource::{Datasource, RedisDatasource};
use crate::dto::token_request::TokenRequest;
use crate::dto::token_response::TokenResponse;
use crate::errors::api_error::ApiError;
use crate::errors::auth_error::AuthError;
use crate::hash_helpers::verify_password;
use crate::middlewares::auth::TokenAuth;
use crate::models::user::{ReqNewUser, ReqUpdUser};
use crate::repository::common::Repository;
use crate::repository::user::UserRepository;
use crate::Config;

#[macro_export]
macro_rules! generate_token {
    ($iss: ident, $user_id: expr, $aud: expr, $exp: expr, $secret: expr) => {
        match generate_token($iss, $user_id, $aud, $exp, $secret) {
            Ok(t) => t,
            Err(e) => return AuthError::Other(String::from(e.to_string())).error_response(),
        }
    };
}

fn generate_token(
    iss: &str,
    user_id: i32,
    aud: &str,
    exp: u64,
    secret: &str,
) -> Result<String, ApiError> {
    use hmac::{Hmac, NewMac};
    use jwt::{RegisteredClaims, SignWithKey};
    use sha2::Sha256;

    let claims = RegisteredClaims {
        issuer: Some(iss.into()),
        subject: Some(user_id.to_string()),
        audience: Some(aud.into()),
        expiration: Some(exp.into()),
        not_before: None,
        issued_at: Some(chrono::Utc::now().timestamp() as u64),
        json_web_token_id: None,
    };

    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())
        .map_err(|_e| ApiError::internal_server_error(Some("Invalid key".to_string())))?;

    let signed_token = claims
        .sign_with_key(&key)
        .map_err(|_e| ApiError::internal_server_error(Some("Sign failed".to_string())))?;

    Ok(signed_token)
}

pub async fn login(
    app_state: web::Data<Config>,
    user_repo: web::Data<UserRepository>,
    req: web::Json<TokenRequest>,
) -> HttpResponse {
    use chrono::{Duration, Utc};

    let issuer = app_state.issuer.as_str();
    let secret = app_state.secret.as_str();
    let audience = app_state.audience.as_str();

    if let Some(admin) = app_state.admin_username.clone() {
        if let Some(password) = app_state.admin_password.clone() {
            if req.username == admin && req.password == password {
                let access_token = generate_token!(
                    issuer,
                    -1,
                    audience,
                    (Utc::now() + Duration::hours(1)).timestamp() as u64,
                    secret
                );
                let refresh_token = generate_token!(
                    issuer,
                    -1,
                    audience,
                    (Utc::now() + Duration::days(1)).timestamp() as u64,
                    secret
                );
                return HttpResponse::Ok().json(TokenResponse {
                    access_token,
                    refresh_token,
                });
            }
        }
    }

    let result = match user_repo.get_by_username(req.username.clone().as_str()) {
        Ok(res) => res,
        Err(_) => return AuthError::InvalidUsername.error_response(),
    };

    if !result.2 {
        return AuthError::NotActive.error_response();
    }

    if verify_password(req.password.as_str(), result.1.as_str()) {
        let access_token = generate_token!(
            issuer,
            result.0,
            audience,
            (Utc::now() + Duration::hours(1)).timestamp() as u64,
            secret
        );
        let refresh_token = generate_token!(
            issuer,
            result.0,
            audience,
            (Utc::now() + Duration::days(1)).timestamp() as u64,
            secret
        );
        HttpResponse::Ok().json(TokenResponse {
            access_token,
            refresh_token,
        })
    } else {
        AuthError::InvalidPassword.error_response()
    }
}

pub async fn logout(
    token_auth: TokenAuth,
    redis_datasource: web::Data<RedisDatasource>,
) -> HttpResponse {
    use redis::Commands;
    let mut client = redis_datasource.get_client();

    match client.rpush::<&str, &str, i32>("token_blacklist", token_auth.token.unwrap().as_str()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => ApiError::internal_server_error(Some(e.to_string())).error_response(),
    }
}

pub async fn register(
    user_repo: web::Data<UserRepository>,
    obj: web::Json<ReqNewUser>,
) -> HttpResponse {
    match user_repo.into_inner().create(obj.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => e.error_response(),
    }
}

pub async fn find_all(token_auth: TokenAuth, user_repo: web::Data<UserRepository>) -> HttpResponse {
    match user_repo.find_all() {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => e.error_response(),
    }
}

pub async fn create(
    token_auth: TokenAuth,
    user_repo: web::Data<UserRepository>,
    obj: web::Json<ReqNewUser>,
) -> HttpResponse {
    match user_repo.create_active(obj.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => e.error_response(),
    }
}

pub async fn update(
    token_auth: TokenAuth,
    user_repo: web::Data<UserRepository>,
    id: web::Path<(i32,)>,
    obj: web::Json<ReqUpdUser>,
) -> HttpResponse {
    match user_repo.update(id.into_inner().0, obj.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => e.error_response(),
    }
}

pub async fn delete(
    token_auth: TokenAuth,
    user_repo: web::Data<UserRepository>,
    id: web::Path<(i32,)>,
) -> HttpResponse {
    match user_repo.delete(id.into_inner().0) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => e.error_response(),
    }
}
