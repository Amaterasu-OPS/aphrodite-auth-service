use std::sync::Arc;
use crate::application::spi::db::DBInterface;

#[allow(unused)]
pub trait RepositoryInterface {
    type DB: DBInterface;

    type Model;
    type Id;
    
    fn new(table_name: String, pool: Arc<Self::DB>) -> Self;

    async fn insert(&self, data: Self::Model) -> Result<Self::Model, String>;
    async fn count(&self) -> i32;
    async fn list(&self, page: i32, limit: i32) -> Vec<Self::Model>;
    async fn edit(&self, id: Self::Id, data: Self::Model, fields: Vec<String>) -> Result<Self::Model, String>;
    async fn get(&self, id: Self::Id) -> Result<Self::Model, String>;
    async fn delete(&self, id: Self::Id) -> Result<Self::Id, String>;
}