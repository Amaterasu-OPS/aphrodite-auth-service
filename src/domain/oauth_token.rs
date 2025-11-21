#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct OauthToken {
    pub id: Option<uuid::Uuid>,
    pub session_id: Option<uuid::Uuid>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub status: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}