use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub scopes: Vec<String>,
    pub sub: uuid::Uuid,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: String,
    pub jti: String,
    pub sid: String,
    pub client_id: String,
    pub auth_time: usize
}