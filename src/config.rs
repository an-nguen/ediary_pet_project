use rocket::figment::Figment;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub redis_address: String,
    pub redis_module: String,
    pub hash_secret_key: String,
    pub token_secret_key: String,
    #[serde(skip_serializing)]
    pub admin: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
}

impl AppConfig {
    pub fn new_figment(figment: &Figment) -> Self {
        let mut config = figment.extract_inner::<Self>("app").unwrap();
        if let Ok(admin) = env::var("ADMIN") {
            config.admin = Some(admin);
        }
        if let Ok(password) = env::var("PASSWORD") {
            config.password = Some(password);
        }

        return config;
    }
}
