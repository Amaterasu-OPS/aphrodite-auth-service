#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct OauthClient {
    pub id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub secret: Option<String>,
    pub urls: Option<Vec<String>>,
    pub scopes: Option<Vec<String>>,
    pub status: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}