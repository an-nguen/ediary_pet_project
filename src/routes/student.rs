use rocket::serde::json::Json;

use crate::models::errors::ApiError;
use crate::models::student::{NewStudent, Student, UpdStudent};
use crate::models::DeletedCount;
use crate::repository::student;
use crate::routes::RouteResult;
use crate::token::Token;
use crate::MainDb;

#[get("/")]
pub async fn find_all(conn: MainDb) -> RouteResult<Vec<Student>> {
    conn.run(|c| match crate::repository::student::find_all(&c) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    })
    .await
}

#[post("/", data = "<student>")]
pub async fn create(
    db: MainDb,
    token: Token<'_>,
    student: Json<NewStudent>,
) -> RouteResult<Student> {
    user_has_role!(token, "ADMIN");
    db.run(|c| match student::create(c, student.into_inner()) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err),
    })
    .await
}

#[put("/<id>", data = "<student>")]
pub async fn update(
    db: MainDb,
    token: Token<'_>,
    id: i32,
    student: Json<UpdStudent>,
) -> RouteResult<Student> {
    user_has_role!(token, "ADMIN");
    db.run(
        move |c| match student::update(c, id, student.into_inner()) {
            Ok(res) => Ok(Json(res)),
            Err(e) => Err(e),
        },
    )
    .await
}

#[delete("/<id>")]
pub async fn delete(db: MainDb, token: Token<'_>, id: i32) -> RouteResult<DeletedCount> {
    user_has_role!(token, "ADMIN");
    db.run(move |c| match student::delete(c, id) {
        Ok(dc) => Ok(Json(dc)),
        Err(e) => Err(e),
    })
    .await
}
