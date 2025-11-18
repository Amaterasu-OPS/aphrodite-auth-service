#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct OauthSession {
    pub id: Option<uuid::Uuid>,
    pub client_id: Option<String>,
    pub user_id: Option<uuid::Uuid>,
    pub scopes: Option<Vec<String>>,
    pub redirect_uri: Option<String>,
    pub state: Option<String>,
    pub response_type: Option<String>,
    pub authorization_code: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
    pub status: Option<i32>,
    pub consent_granted_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}