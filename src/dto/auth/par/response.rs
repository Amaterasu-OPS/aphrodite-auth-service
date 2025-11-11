#[derive(Debug, serde::Serialize)]
pub struct ParResponse {
    pub request_uri: String,
    pub expires_in: u64,
}