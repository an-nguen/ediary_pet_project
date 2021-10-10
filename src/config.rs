use actix_web::web;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub url: String,
    pub port: u16,
    pub secret_key: String,
    pub resource_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub pg_uri: String,
    pub redis_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub admin_username: Option<String>,
    pub admin_password: Option<String>,
}

impl Settings {
    pub fn new(filepath: &str) -> Self {
        use config::Config;

        let mut c = Config::default();
        c.merge(config::File::with_name(filepath))
            .expect("config file not found");
        c.merge(config::Environment::with_prefix("APP")).unwrap();
        c.try_into().expect("failed to parse config")
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    use crate::handlers::*;

    cfg.route("/login", web::post().to(user::login))
        .route("/logout", web::post().to(user::logout))
        .route("/register", web::post().to(user::register))
        .service(
            web::scope("/api/v1")
                .service(
                    web::scope("/user")
                        .service(
                            web::resource("")
                                .route(web::get().to(user::find_all))
                                .route(web::post().to(user::create)),
                        )
                        .service(
                            web::resource("/{id}")
                                .route(web::put().to(user::update))
                                .route(web::delete().to(user::delete)),
                        ),
                )
                .service(
                    web::scope("/student")
                        .service(
                            web::resource("")
                                .route(web::get().to(student::find_all))
                                .route(web::post().to(student::create)),
                        )
                        .service(
                            web::resource("/{id}")
                                .route(web::put().to(student::update))
                                .route(web::delete().to(student::delete)),
                        ),
                )
                .service(
                    web::scope("/subject")
                        .service(
                            web::resource("")
                                .route(web::get().to(subject::find_all))
                                .route(web::post().to(subject::create)),
                        )
                        .service(
                            web::resource("/{id}")
                                .route(web::put().to(subject::update))
                                .route(web::delete().to(subject::delete)),
                        ),
                ),
        );
}
