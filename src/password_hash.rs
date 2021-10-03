use crate::AppConfig;
use argon2::{ThreadMode, Variant, Version};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct PasswordHashService {
    pub secret_key: String,
}

impl PasswordHashService {
    pub fn config(&self) -> argon2::Config {
        argon2::Config {
            variant: Variant::Argon2i,
            version: Version::Version13,
            mem_cost: 65536,
            time_cost: 10,
            lanes: 4,
            thread_mode: ThreadMode::Parallel,
            secret: self.secret_key.as_bytes(),
            ad: &[],
            hash_length: 128,
        }
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
    pub fn verify_password(&self, password: &str, salt: &str, hash: &str) -> bool {
        argon2::verify_raw(
            password.as_bytes(),
            salt.as_bytes(),
            hash.as_bytes(),
            &self.config(),
        )
        .unwrap()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PasswordHashService {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let app_config = request
            .rocket()
            .state::<AppConfig>()
            .expect("token service is not defined");

        Outcome::Success(PasswordHashService {
            secret_key: app_config.hash_secret_key.clone(),
        })
    }
}
