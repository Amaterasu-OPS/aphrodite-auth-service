#[derive(Debug, serde::Serialize)]
pub struct ConsentInfoResponse {
    pub client_id: String,
    pub name: String,
    pub scopes: Vec<String>,
    pub mandatory_scopes: Vec<String>,
    pub created_at: chrono::NaiveDateTime,
}