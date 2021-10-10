use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use r2d2::PooledConnection;

pub trait Datasource<T> {
    fn new(url: &str, size: u32) -> Self;
    fn get_client(&self) -> T;
}

#[derive(Clone)]
pub struct RedisDatasource {
    pub pool: Pool<redis::Client>,
}

impl Datasource<PooledConnection<redis::Client>> for RedisDatasource {
    fn new(url: &str, size: u32) -> Self {
        let client: redis::Client = redis::Client::open(url).unwrap();
        let pool = r2d2::Pool::<redis::Client>::builder()
            .max_size(size)
            .build(client)
            .unwrap();
        RedisDatasource { pool }
    }
    fn get_client(&self) -> PooledConnection<redis::Client> {
        self.pool.get().unwrap()
    }
}

pub fn new_pool_pg(url: &str, size: u32) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder().max_size(size).build(manager).unwrap()
}
