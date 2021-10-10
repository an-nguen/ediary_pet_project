use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, Serialize, Clone)]
pub struct ApiError {
    pub status_code: u16,
    pub error_message: Option<String>,
}

impl ApiError {
    #[warn(dead_code)]
    pub fn new(status_code: u16, error_message: Option<String>) -> Self {
        ApiError {
            status_code,
            error_message,
        }
    }
    pub fn internal_server_error(error_message: Option<String>) -> Self {
        ApiError {
            status_code: 500,
            error_message,
        }
    }
    pub fn bad_request(error_message: Option<String>) -> Self {
        ApiError {
            status_code: 400,
            error_message,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ApiError {}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code).unwrap()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
