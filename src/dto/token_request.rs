#[derive(Deserialize, Debug, Clone)]
pub struct TokenRequest {
    pub username: String,
    pub password: String,
}
