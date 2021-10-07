use crate::models::errors::ApiError;
use rocket::serde::json::Json;

#[macro_use]
pub mod macros;
pub mod student;
pub mod subject;
pub mod user;

pub type RouteResult<T> = Result<Json<T>, ApiError>;
