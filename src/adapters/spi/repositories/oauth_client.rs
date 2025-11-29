use std::sync::Arc;
use crate::adapters::spi::db::postgres_db::PostgresDB;
use crate::application::spi::repository::RepositoryInterface;
use crate::domain::oauth_client::OauthClient;
use crate::for_each_field;

pub struct OAuthClientRepository {
    db: Arc<PostgresDB>,
    table: String,
}

#[allow(unused)]
impl RepositoryInterface for OAuthClientRepository {
    type DB = PostgresDB;
    type Model = OauthClient;
    type Id = uuid::Uuid;

    fn new(table_name: String, pool: Arc<Self::DB>) -> Self {
        OAuthClientRepository {
            db: pool,
            table: table_name,
        }
    }

    async fn insert(&self, data: Self::Model) -> Result<Self::Model, String> {
        let query = format!(r#"
            INSERT INTO {} (
                name,
                slug,
                urls,
                scopes
            ) VALUES ($1, $2, $3, $4) RETURNING id
            "#, self.table.clone());

        let insert_result = sqlx::query_scalar::<_, uuid::Uuid>(&query)
            .bind(data.name)
            .bind(data.slug)
            .bind(data.urls)
            .bind(data.scopes)
            .fetch_one(&self.db.pool)
            .await;

        let id = match insert_result {
            Ok(id) => id,
            Err(_) => {
                return Err(String::from("Failed to insert client"))
            }
        };

        match sqlx::query_as::<_, Self::Model>(format!("SELECT * FROM {} WHERE id = $1", self.table.clone()).as_str())
            .bind(id)
            .fetch_one(&self.db.pool).await {
            Ok(e) => Ok(e),
            Err(_) => Err(String::from("Cannot retrieve client"))
        }
    }

    async fn count(&self) -> i32 {
        let query = format!("SELECT count(*) AS TOTAL FROM {}", self.table.clone());

        sqlx::query_scalar::<_, i32>(&query)
            .fetch_one(&self.db.pool)
            .await.unwrap()
    }

    async fn list(&self, page: i32, limit: i32) -> Vec<Self::Model> {
        let query = format!("SELECT * FROM {} LIMIT $1 OFFSET $2", self.table.clone());

        sqlx::query_as::<_, Self::Model>(&query)
            .bind(limit)
            .bind(page * limit)
            .fetch_all(&self.db.pool)
            .await.unwrap()
    }

    async fn edit(
        &self,
        id: Self::Id,
        data: Self::Model,
        fields: Vec<&str>,
    ) -> Result<Self::Model, String> {
        let value = serde_json::to_value(&data)
            .map_err(|_| String::from("Failed to serialize model for update"))?;

        let mut query =
            sqlx::QueryBuilder::new(format!("UPDATE {} SET ", self.table));

        let mut set_clauses = query.separated(", ");

        for_each_field!(data, { name, urls, scopes, status }, |k: &str, v| {
            if fields.contains(&k) {
                set_clauses.push(format!(" {} = ", k));
                set_clauses.push_bind_unseparated(v);
            }
        });

        query.push(" WHERE id = ");
        query.push_bind(id);

        query
            .build()
            .execute(&self.db.pool)
            .await
            .map_err(|_| String::from("Failed to update client"))?;

        self.get(id).await
    }

    async fn get(&self, id: Self::Id) -> Result<Self::Model, String> {
        let query = format!("SELECT * FROM {} WHERE id = $1", self.table.clone());

        match sqlx::query_as::<_, Self::Model>(&query)
            .bind(id)
            .fetch_one(&self.db.pool)
            .await {
            Ok(e) => Ok(e),
            Err(_) => Err(String::from("Client not found"))
        }
    }

    async fn delete(&self, id: Self::Id) -> Result<Self::Id, String> {
        match sqlx::query(&format!("DELETE FROM {} WHERE id = $1", self.table.clone()))
            .bind(id)
            .execute(&self.db.pool)
            .await {
            Ok(_) => Ok(id),
            Err(_) => Err(String::from("Client not found"))
        }
    }
}

#[allow(unused)]
impl OAuthClientRepository {
    pub async fn get_by_slug_secret(&self, slug: String, secret: String) -> Result<OauthClient, String> {
        let query = format!("SELECT * FROM {} WHERE slug = $1 and secret = $2", self.table.clone());

        match sqlx::query_as::<_, OauthClient>(&query)
            .bind(slug)
            .bind(secret)
            .fetch_one(&self.db.pool)
            .await {
            Ok(e) => Ok(e),
            Err(_) => Err(String::from("Client not found"))
        }
    }

    pub async fn get_by_slug(&self, slug: String) -> Result<OauthClient, String> {
        let query = format!("SELECT * FROM {} WHERE slug = $1", self.table.clone());

        match sqlx::query_as::<_, OauthClient>(&query)
            .bind(slug)
            .fetch_one(&self.db.pool)
            .await {
            Ok(e) => Ok(e),
            Err(_) => Err(String::from("Client not found"))
        }
    }
}