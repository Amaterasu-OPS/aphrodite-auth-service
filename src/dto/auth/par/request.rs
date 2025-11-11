#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ParRequest {
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}