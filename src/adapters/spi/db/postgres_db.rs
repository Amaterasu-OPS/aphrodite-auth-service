use sqlx::{Pool, Postgres};
use crate::application::spi::db::{DBFactory, DBInterface};

#[derive(Clone, Debug)]
pub struct PostgresDB {
    pub pool: Pool<Postgres>,
}

impl DBFactory for PostgresDB {
    async fn get() -> Result<Self, sqlx::Error> {
        Ok(PostgresDB::new().await)
    }
}

impl DBInterface for PostgresDB {
    type DB = Postgres;
    type T = Pool<Postgres>;

    async fn connect(user: String, password: String, host: String, port: String, db_name: String) -> Pool<Postgres> {
        let url = format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, db_name);

        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(std::env::var("DB_MAX_POOL_SIZE").unwrap_or(String::from("20")).parse::<u32>().unwrap())
            .acquire_timeout(std::time::Duration::from_secs(std::env::var("DB_CONN_TIMEOUT").unwrap_or(String::from("30")).parse::<u64>().unwrap()))
            .idle_timeout(std::time::Duration::from_secs(std::env::var("DB_IDLE_TIMEOUT").unwrap_or(String::from("600")).parse::<u64>().unwrap()))
            .max_lifetime(std::time::Duration::from_secs(std::env::var("DB_MAX_LIFETIME").unwrap_or(String::from("1800")).parse::<u64>().unwrap()))
            .connect(&url).await {
            Ok(conn) => conn,
            Err(_) => {
                panic!("{}", format!("Cannot connect to {}", url).to_string());
            }
        }
    }

    async fn new() -> Self {
        PostgresDB {
            pool: PostgresDB::connect(
                std::env::var("DB_USER").expect("DB_USER environment variable not set"),
                std::env::var("DB_PASSWORD").expect("DB_PASSWORD environment variable not set"),
                std::env::var("DB_HOST").expect("DB_HOST environment variable not set"),
                std::env::var("DB_PORT").expect("DB_PORT environment variable not set"),
                std::env::var("DB_DB").expect("DB_DB environment variable not set"),
            ).await
        }
    }
}
