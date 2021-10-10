extern crate actix_service;
extern crate actix_web;
extern crate argon2;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate proc_macro;
#[macro_use]
extern crate serde_derive;
extern crate redis;

use crate::config::Settings;
use crate::db::datasource;
use crate::db::datasource::{Datasource, RedisDatasource};
use crate::repository::role::RoleRepository;
use crate::repository::student::StudentRepository;
use crate::repository::subject::SubjectRepository;
use crate::repository::user::UserRepository;
use actix_web::http::ContentEncoding;
use actix_web::middleware::TrailingSlash;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use std::fs;

mod config;
mod db;
mod dto;
mod errors;
mod handlers;
mod hash_helpers;
mod middlewares;
pub mod models;
mod repository;
mod schema;
#[cfg(test)]
mod tests;

pub struct Config {
    pub admin_username: Option<String>,
    pub admin_password: Option<String>,
    pub issuer: String,
    pub audience: String,
    pub secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let key_content = fs::read("./tls/key.pkcs8").unwrap();
    let cert_content = fs::read("./tls/cert.der").unwrap();

    // Load TLS key and cert files
    let key = PrivateKey(key_content);
    let cert = Certificate(cert_content);

    let mut tls_config = ServerConfig::new(NoClientAuth::new());

    tls_config.set_single_cert(vec![cert], key).unwrap();

    let settings = Settings::new("./configs/Default.toml");
    let server_url = settings.server.url.clone();
    let audience = format!("{}{}", settings.server.url, settings.server.resource_path);

    let pool_pg = datasource::new_pool_pg(settings.database.pg_uri.as_str(), 4);
    let user_repo = UserRepository {
        pg_pool: pool_pg.clone(),
    };
    let subj_repo = SubjectRepository(pool_pg.clone());
    let role_repo = RoleRepository(pool_pg.clone());
    let student_repo = StudentRepository(pool_pg.clone());
    let redis_datasource = RedisDatasource::new(settings.database.redis_uri.as_str(), 4);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Config {
                secret: settings.server.secret_key.clone(),
                issuer: settings.server.url.clone(),
                admin_username: settings.admin_username.clone(),
                admin_password: settings.admin_password.clone(),
                audience: audience.clone(),
            }))
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(subj_repo.clone()))
            .app_data(web::Data::new(student_repo.clone()))
            .app_data(web::Data::new(role_repo.clone()))
            .app_data(web::Data::new(redis_datasource.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .configure(config::configure_routes)
    })
    .bind_rustls(
        &format!("{}:{}", server_url, settings.server.port),
        tls_config,
    )?
    .run()
    .await
}
