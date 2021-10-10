use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum AuthError {
    InvalidUsername,
    InvalidPassword,
    NotActive,
    MissingAuthHeader,
    InvalidToken,
    TokenExpired,
    AccessDenied,
    Other(String),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AuthError {}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingAuthHeader => StatusCode::BAD_REQUEST,
            AuthError::AccessDenied => StatusCode::FORBIDDEN,
            AuthError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
