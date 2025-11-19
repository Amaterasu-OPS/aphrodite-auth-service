use deadpool_redis::{Config, Pool, Connection};
use crate::application::spi::cache::{CacheFactory, CacheInterface};

#[derive(Clone, Debug)]
pub struct RedisCache {
    pub pool: Pool,
}

impl CacheInterface for RedisCache {
    type T = Pool;

    fn connect(host: String, port: String, db: String) -> Pool {
        let cfg = Config::from_url(format!("redis://{}:{}/{}", host, port, db));
        cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).expect("Cannot create Redis pool")
    }

    fn new() -> Self {
        Self {
            pool: Self::connect(
                std::env::var("REDIS_HOST").expect("REDIS_HOST environment variable not set"),
                std::env::var("REDIS_PORT").expect("REDIS_PORT environment variable not set"),
                std::env::var("REDIS_DB").expect("REDIS_DB environment variable not set")
            ),
        }
    }
}

impl RedisCache {
    pub async fn get_pool(&self) -> Result<Connection, String> {
        match self.pool.get()
            .await {
            Ok(conn) => Ok(conn),
            Err(_) => Err(String::from("Failed to get connection from pool"))
        }
    }
}

impl CacheFactory for RedisCache {
    fn get() -> Result<Self, String> {
        Ok(RedisCache::new())
    }
}