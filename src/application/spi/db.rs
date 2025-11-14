use sqlx::{Database};

pub trait DBInterface: Send + Sync {
    type DB: Database;

    type T;

    async fn connect(user: String, password: String, host: String, port: String, db_name: String) -> Self::T where Self: Sized;
    async fn new() -> Self where Self: Sized;
}

pub trait DBFactory: Sized {
    async fn get() -> Result<Self, sqlx::Error>;
}