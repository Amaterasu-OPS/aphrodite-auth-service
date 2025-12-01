#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct OauthConsent {
    pub id: Option<uuid::Uuid>,
    pub client_id: Option<String>,
    pub user_id: Option<uuid::Uuid>,
    pub scopes: Option<Vec<String>>,
    pub status: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}