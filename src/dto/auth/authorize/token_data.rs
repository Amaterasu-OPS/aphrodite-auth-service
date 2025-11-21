use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    pub user_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}