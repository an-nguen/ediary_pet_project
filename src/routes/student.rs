use rocket::serde::json::Json;

use crate::models::error::ApiError;
use crate::models::student::{NewStudent, Student};
use crate::repository::student;
use crate::DbConn;

#[get("/")]
pub async fn find_all(conn: DbConn) -> Json<Vec<Student>> {
    Json(
        conn.run(|c| crate::repository::student::find_all(&c).unwrap())
            .await,
    )
}

#[post("/", data = "<student>")]
pub async fn create(db: DbConn, student: Json<NewStudent>) -> Result<Json<Student>, ApiError> {
    db.run(|c| match student::create(c, student.into_inner()) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err),
    })
    .await
}
