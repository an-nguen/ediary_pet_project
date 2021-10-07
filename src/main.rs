extern crate argon2;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate serde_derive;
extern crate redis;

use dotenv::dotenv;
use rocket_sync_db_pools::database;

use crate::config::AppConfig;
use crate::rocket_redis::RedisDb;

mod config;
pub mod models;
mod password_hash;
mod repository;
mod rocket_redis;
mod routes;
mod schema;
mod tests;
mod token;

#[database("main_db")]
pub struct MainDb(diesel::PgConnection);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    use routes::{student, subject, user};

    dotenv().expect(".env file not found");

    let rocket = rocket::build();
    let figment = rocket.figment();
    let config = AppConfig::new_figment(figment);
    let redis_pool = RedisDb::new_pool(&config.redis_address);

    rocket
        .manage(config)
        .manage(redis_pool)
        .mount("/", routes![user::login, user::register, user::logout])
        .mount(
            "/api/user",
            routes![user::find_all, user::create, user::update, user::delete],
        )
        .mount(
            "/api/student",
            routes![
                student::find_all,
                student::create,
                student::update,
                student::delete
            ],
        )
        .mount(
            "/api/subject",
            routes![
                subject::find_all,
                subject::create,
                subject::update,
                subject::delete
            ],
        )
        .attach(MainDb::fairing())
        .launch()
        .await
}
