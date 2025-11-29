#[derive(Debug, serde::Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
}