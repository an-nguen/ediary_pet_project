use crate::db::datasource::RedisDatasource;
use crate::errors::auth_error::AuthError;
use crate::models::role::Role;
use crate::repository::role::RoleRepository;
use crate::Config;
use actix_web::dev::Payload;
use actix_web::error::Error;
use actix_web::{web, FromRequest, HttpRequest};
use hmac::{Hmac, NewMac};
use jwt::{RegisteredClaims, VerifyWithKey};
use redis::Commands;
use sha2::Sha256;
use std::future::{ready, Ready};

/// example https://docs.rs/crate/actix-session/0.3.0/source/src/lib.rs
///

pub struct TokenAuth {
    pub role_repo: RoleRepository,
    pub redis_db: RedisDatasource,
    pub token: Option<String>,
    key: Hmac<Sha256>,
    issuer: String,
    audience: String,
    pub roles: Vec<Role>,
}

impl TokenAuth {
    const BEARER: &'static str = "Bearer";

    pub fn new(
        config: &web::Data<Config>,
        role_repo: RoleRepository,
        redis_db: RedisDatasource,
    ) -> Self {
        TokenAuth {
            role_repo,
            redis_db,
            token: None,
            key: Hmac::new_from_slice(config.secret.clone().as_bytes()).unwrap(),
            issuer: config.issuer.clone(),
            audience: config.audience.clone(),
            roles: vec![],
        }
    }

    pub fn extract_token(req: &HttpRequest) -> Result<String, AuthError> {
        use actix_web::http::header;

        if let Some(header_value) = req.headers().get(header::AUTHORIZATION) {
            let header_value_str = match header_value.to_str() {
                Ok(v) => String::from(v),
                Err(e) => return Err(AuthError::Other(e.to_string())),
            };
            if header_value_str.starts_with(TokenAuth::BEARER)
                && header_value_str.len() > (TokenAuth::BEARER.len() + 1)
            {
                let token = &header_value_str[TokenAuth::BEARER.len() + 1..];
                return Ok(String::from(token));
            }
        }
        return Err(AuthError::MissingAuthHeader);
    }

    pub fn verify_request(&mut self, req: &HttpRequest) -> Result<bool, AuthError> {
        use chrono::Utc;

        let token = Self::extract_token(req).map_err(|_| AuthError::InvalidToken)?;
        self.token = Some(token.clone());
        let mut client = self
            .redis_db
            .pool
            .get()
            .map_err(|e| AuthError::Other(e.to_string()))?;

        let claims: RegisteredClaims = match token.as_str().verify_with_key(&self.key) {
            Ok(r) => r,
            Err(_) => return Err(AuthError::InvalidToken),
        };
        let blacklist: Vec<String> = client
            .lrange("token_blacklist", 0, -1)
            .map_err(|e| AuthError::Other(e.to_string()))?;
        if blacklist.iter().any(|t| t == &token) {
            return Err(AuthError::InvalidToken);
        }
        let user_id = claims
            .subject
            .clone()
            .unwrap()
            .parse::<i32>()
            .map_err(|_| AuthError::InvalidToken)?;
        if user_id != -1 {
            self.roles = self.role_repo.get_roles_by_user_id(user_id);
        } else {
            self.roles = vec![Role {
                id: -1,
                name: "ADMIN".to_string(),
                description: None,
            }]
        }

        let is_iss_valid = if let Some(iss) = claims.issuer {
            iss == self.issuer
        } else {
            false
        };

        let is_aud_valid = if let Some(aud) = claims.audience {
            aud == self.audience
        } else {
            false
        };

        let is_not_expired = match claims.expiration {
            Some(exp) => exp > Utc::now().timestamp() as u64,
            None => return Err(AuthError::TokenExpired),
        };

        Ok(is_iss_valid && is_not_expired && is_aud_valid)
    }

    pub fn has_role(&self, role_name: &str) -> bool {
        self.roles.iter().any(|r| r.name == role_name)
    }
}

impl FromRequest for TokenAuth {
    type Config = ();
    type Error = actix_web::error::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cfg = req.app_data::<web::Data<Config>>().unwrap();
        let role_repo = req.app_data::<web::Data<RoleRepository>>().unwrap();
        let redis_datasource = req.app_data::<web::Data<RedisDatasource>>().unwrap();
        let mut auth = TokenAuth::new(
            cfg,
            role_repo.get_ref().clone(),
            redis_datasource.get_ref().clone(),
        );
        println!("{}", req.path());
        match auth.verify_request(req) {
            Ok(is_valid) => ready(if is_valid {
                Ok(auth)
            } else {
                Err(Error::from(AuthError::InvalidToken))
            }),
            Err(e) => ready(Err(Error::from(e))),
        }
    }
}

// pub struct Auth;
//
// impl<S, B> Transform<S, ServiceRequest> for Auth
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::error::Error>,
//     S::Future: 'static,
//     B: MessageBody,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Transform = AuthMiddleware<S>;
//     type InitError = ();
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(AuthMiddleware { service }))
//     }
// }
//
// pub struct AuthMiddleware<S> {
//     pub service: S,
// }
//
// impl<S> AuthMiddleware<S> {
//     const BEARER: &'static str = "Bearer";
//
//     pub fn get_token(&self, req: &ServiceRequest) -> Result<String, std::io::Error> {
//         use actix_web::http::header;
//         use std::io::Error;
//
//         let val = match req.headers().get(header::AUTHORIZATION) {
//             Some(val) => val,
//             None => {
//                 return Err(Error::new(
//                     ErrorKind::Other,
//                     "failed to get authorization value",
//                 ))
//             }
//         };
//         let header_value = val.to_str().unwrap();
//
//         if String::from(header_value).starts_with(AuthMiddleware::<S>::BEARER)
//             && header_value.len() > AuthMiddleware::<S>::BEARER.len() + 1
//         {
//             Ok(String::from(
//                 &header_value[AuthMiddleware::<S>::BEARER.len() + 1..],
//             ))
//         } else {
//             Err(Error::new(
//                 ErrorKind::Other,
//                 "failed to get parse authorization value",
//             ))
//         }
//     }
// }
//
// impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::error::Error>,
//     S::Future: 'static,
//     B: MessageBody,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
//
//     actix_service::forward_ready!(service);
//
//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         let datasource = req.app_data::<web::Data<Datasource>>().unwrap();
//
//         let token = match self.get_token(&req) {
//             Ok(t) => t,
//             Err(e) => {
//                 return Box::pin(async move {
//                     Err(Error::from(ApiError::new(401, Some(e.to_string()))))
//                 })
//             }
//         };
//
//         let fut = self.service.call(req);
//
//         Box::pin(async move {
//             let res = fut.await?;
//             println!("{}", token);
//             Ok(res)
//         })
//     }
// }
