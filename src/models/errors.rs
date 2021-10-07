use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use std::io::Cursor;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum AuthError {
    InvalidUsername,
    InvalidPassword,
    NotActive,
    MissingAuthHeader,
    InvalidToken,
    AccessDenied,
    Other(String),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AuthError {}

#[derive(Debug, Serialize, Clone)]
pub struct ApiError {
    pub status_code: u16,
    pub error_message: Option<String>,
}

impl ApiError {
    pub fn new(status_code: u16, error_message: Option<String>) -> Self {
        ApiError {
            status_code,
            error_message,
        }
    }

    pub fn bad_request(error_message: Option<String>) -> Self {
        ApiError {
            status_code: Status::BadRequest.code,
            error_message,
        }
    }

    pub fn internal_server_error(error_message: Option<String>) -> Self {
        ApiError {
            status_code: Status::InternalServerError.code,
            error_message,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _req: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let mut build = rocket::Response::build();
        if let Ok(msg) = serde_json::to_string(&self) {
            build.streamed_body(Cursor::new(msg));
        }
        build
            .status(Status::new(self.status_code))
            .header(ContentType::JSON)
            .ok()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ApiError {}
