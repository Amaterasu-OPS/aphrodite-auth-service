use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserinfoResponse {
    pub sub: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub gender: String,
    pub email: String,
    pub created_at: String,
}
