use crate::AppConfig;
use argon2::Config;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct PasswordHashService {
    pub secret_key: String,
}

impl PasswordHashService {
    pub fn config(&self) -> Config {
        argon2::Config::default()
    }

    pub fn generate_salt() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }

    pub fn hash_password(&self, password: &str, salt: &str) -> String {
        argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &self.config())
            .expect("failed to hash password")
    }
    pub fn verify_password(&self, password: &str, hash: &str) -> bool {
        argon2::verify_encoded(hash, password.as_bytes()).unwrap()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PasswordHashService {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = request
            .rocket()
            .state::<AppConfig>()
            .expect("argon2::Config is not defined");

        Outcome::Success(PasswordHashService {
            secret_key: config.hash_secret_key.clone(),
        })
    }
}
