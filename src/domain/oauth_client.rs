use sqlx::types::Json;

#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct OauthClient {
    pub id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub secret: Option<String>,
    pub urls: Option<Vec<String>>,
    pub scopes: Option<Vec<String>>,
    pub mandatory_scopes: Option<Vec<String>>,
    pub status: Option<i32>,
    pub logos: Option<Json<Vec<String>>>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}