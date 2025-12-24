#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserinfoRequest {
    pub sub: String,
}