use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthorizeRequest {
    pub client_id: Option<String>,
    pub uri: Option<String>,
    pub session_id: Option<String>,
    pub user_id: Option<Uuid>,
    pub auth_token: Option<String>,
    pub consent_id: Option<Uuid>,
}
