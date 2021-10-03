use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use rocket::http::Status;
use rocket::response::Responder;

#[derive(Debug)]
pub struct MyError {
    pub details: String,
}

impl MyError {
    pub fn new(details: &str) -> MyError {
        MyError {
            details: String::from(details),
        }
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        self.details.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    status_code: u16,
    error_message: Option<String>,
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
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let mut build = rocket::Response::build();
        if let Some(responder) = self.error_message {
            build.merge(responder.respond_to(req)?);
        }

        build.status(Status::new(self.status_code)).ok()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ApiError {}
