use redis::Commands;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::models::errors::ApiError;
use crate::models::token_request::TokenRequest;
use crate::models::token_response::TokenResponse;
use crate::models::user::{ReqNewUser, ReqUpdUser, UserRead};
use crate::models::DeletedCount;
use crate::password_hash::PasswordHashService;
use crate::rocket_redis::RedisDb;
use crate::routes::RouteResult;
use crate::token::{Token, TokenService};
use crate::{repository, MainDb};

#[post("/login", data = "<token_req>")]
pub async fn login(
    conn: MainDb,
    token_service: TokenService,
    password_hash_service: PasswordHashService,
    token_req: Json<TokenRequest>,
) -> RouteResult<TokenResponse> {
    conn.run(move |c| {
        match repository::user::authenticate(
            c,
            &token_service,
            &password_hash_service,
            token_req.into_inner(),
        ) {
            Ok(response) => Ok(Json(response)),
            Err(err) => Err(ApiError::new(
                Status::Unauthorized.code,
                Some(format!("{:?}", err)),
            )),
        }
    })
    .await
}

// #[post("/refresh")]
// pub async fn refresh() {}

#[post("/register", data = "<user>")]
pub async fn register(
    conn: MainDb,
    password_hash_service: PasswordHashService,
    user: Json<ReqNewUser>,
) -> RouteResult<UserRead> {
    conn.run(move |c| {
        match repository::user::create(c, &password_hash_service, user.into_inner(), false) {
            Ok(u) => Ok(Json(u)),
            Err(e) => Err(e),
        }
    })
    .await
}

#[post("/logout")]
pub async fn logout(mut redis_conn: RedisDb, token: Token<'_>) -> Status {
    return if let Ok(_) =
        redis_conn.rpush::<&str, &str, i32>("token_blacklist", token.token.clone())
    {
        Status::Ok
    } else {
        Status::InternalServerError
    };
}

#[get("/")]
pub async fn find_all(conn: MainDb) -> RouteResult<Vec<UserRead>> {
    conn.run(|c| match repository::user::find_all(c) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    })
    .await
}

#[post("/", data = "<user>")]
pub async fn create(
    conn: MainDb,
    token: Token<'_>,
    password_hash_service: PasswordHashService,
    user: Json<ReqNewUser>,
) -> RouteResult<UserRead> {
    user_has_role!(token, "ADMIN");
    conn.run(move |c| {
        match repository::user::create(c, &password_hash_service, user.into_inner(), true) {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err(e),
        }
    })
    .await
}

#[put("/<id>", data = "<user>")]
pub async fn update(
    conn: MainDb,
    token: Token<'_>,
    password_hash_service: PasswordHashService,
    id: i32,
    user: Json<ReqUpdUser>,
) -> RouteResult<UserRead> {
    user_has_role!(token, "ADMIN");
    conn.run(move |c| {
        match repository::user::update(c, &password_hash_service, id, user.into_inner()) {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err(e),
        }
    })
    .await
}

#[delete("/<id>")]
pub async fn delete(conn: MainDb, token: Token<'_>, id: i32) -> RouteResult<DeletedCount> {
    user_has_role!(token, "ADMIN");

    conn.run(move |c| match repository::user::delete(c, id) {
        Ok(dc) => Ok(Json(dc)),
        Err(e) => Err(e),
    })
    .await
}
