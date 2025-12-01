#[derive(Debug, serde::Serialize)]
pub struct ConsentConfirmResponse {
    pub redirect_url: String,
}