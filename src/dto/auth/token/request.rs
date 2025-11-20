#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub grant_type: String,
    pub redirect_uri: String,
    pub code_verifier: String,
}
