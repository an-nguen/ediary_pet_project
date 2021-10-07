use std::ops::{Deref, DerefMut};

use r2d2::Pool;
use redis::Client;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct RedisDb(pub r2d2::PooledConnection<redis::Client>);

impl RedisDb {
    pub fn new_pool(conn_str: &str) -> Pool<Client> {
        let client = redis::Client::open(conn_str).unwrap();
        r2d2::Pool::builder().build(client).unwrap()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RedisDb {
    type Error = r2d2::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let pool = request
            .rocket()
            .state::<r2d2::Pool<redis::Client>>()
            .unwrap();
        match pool.get() {
            Ok(conn) => Outcome::Success(RedisDb(conn)),
            Err(e) => Outcome::Failure((rocket::http::Status::InternalServerError, e)),
        }
    }
}

impl Deref for RedisDb {
    type Target = r2d2::PooledConnection<redis::Client>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RedisDb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
