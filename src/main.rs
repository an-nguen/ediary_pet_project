extern crate argon2;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate serde_derive;

use rocket::post;
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;

use crate::models::token_request::TokenRequest;
use crate::models::token_response::TokenResponse;
use crate::models::user::{ReqNewUser, UserRead};
use crate::password_hash::PasswordHashService;
use crate::token::TokenService;

pub mod models;
pub mod password_hash;
pub mod repository;
pub mod routes;
pub mod schema;
pub mod token;

#[database("main_db")]
pub struct DbConn(diesel::PgConnection);

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub hash_secret_key: String,
    pub token_secret_key: String,
}

#[post("/login", data = "<token_req>")]
async fn login(
    conn: DbConn,
    token_service: TokenService,
    password_hash_service: PasswordHashService,
    token_req: Json<TokenRequest>,
) -> Result<Json<TokenResponse>, Unauthorized<String>> {
    conn.run(move |c| {
        match repository::user::authenticate(
            c,
            &token_service,
            &password_hash_service,
            token_req.into_inner(),
        ) {
            Ok(response) => Ok(Json(response)),
            Err(err) => Err(Unauthorized(Some(format!("{}", err)))),
        }
    })
    .await
}

#[post("/register", data = "<user>")]
async fn register(
    conn: DbConn,
    password_hash_service: PasswordHashService,
    user: Json<ReqNewUser>,
) -> Json<UserRead> {
    Json(
        conn.run(move |c| {
            repository::user::create(c, &password_hash_service, user.into_inner()).unwrap()
        })
        .await,
    )
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    use routes::student;

    let rocket = rocket::build();
    let figment = rocket.figment();
    let config: AppConfig = figment.extract_inner("app").expect("jwt is not set");

    rocket
        .manage(config)
        .mount("/api/student", routes![student::find_all, student::create])
        .mount("/", routes![crate::login, crate::register])
        .attach(DbConn::fairing())
        .launch()
        .await
}
