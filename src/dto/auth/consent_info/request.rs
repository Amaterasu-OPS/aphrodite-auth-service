use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConsentInfoRequest {
    pub session_id: Uuid,
}