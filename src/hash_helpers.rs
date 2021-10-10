use argon2::Config;

pub fn config() -> Config<'static> {
    Config::default()
}

pub fn generate_random_string(size: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub fn hash_password(pwd: &str) -> String {
    let config = config();

    let salt = generate_random_string(64);
    argon2::hash_encoded(pwd.as_bytes(), salt.as_bytes(), &config).unwrap()
}

pub fn verify_password(pwd: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, pwd.as_bytes()).unwrap()
}
