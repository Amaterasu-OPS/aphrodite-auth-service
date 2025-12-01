use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConsentConfirmRequest {
    pub session_id: Uuid,
    pub scopes: Vec<String>,
}