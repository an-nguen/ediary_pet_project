use crate::models::errors::ApiError;
use crate::models::subject::{Subject, UpdSubject};
use crate::models::DeletedCount;
use crate::repository::subject;
use crate::routes::RouteResult;
use crate::token::Token;
use crate::MainDb;
use rocket::serde::json::Json;

#[get("/")]
pub async fn find_all(conn: MainDb) -> RouteResult<Vec<Subject>> {
    conn.run(|c| match crate::repository::subject::find_all(&c) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    })
    .await
}

#[post("/", data = "<subject>")]
pub async fn create(db: MainDb, token: Token<'_>, subject: Json<Subject>) -> RouteResult<Subject> {
    user_has_role!(token, "ADMIN");
    db.run(|c| match subject::create(c, subject.into_inner()) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err),
    })
    .await
}

#[put("/<id>", data = "<subject>")]
pub async fn update(
    db: MainDb,
    token: Token<'_>,
    id: i32,
    subject: Json<UpdSubject>,
) -> RouteResult<Subject> {
    user_has_role!(token, "ADMIN");

    db.run(
        move |c| match subject::update(c, id, subject.into_inner()) {
            Ok(res) => Ok(Json(res)),
            Err(e) => Err(e),
        },
    )
    .await
}

#[delete("/<id>")]
pub async fn delete(db: MainDb, token: Token<'_>, id: i32) -> RouteResult<DeletedCount> {
    user_has_role!(token, "ADMIN");

    db.run(move |c| match subject::delete(c, id) {
        Ok(dc) => Ok(Json(dc)),
        Err(e) => Err(e),
    })
    .await
}
