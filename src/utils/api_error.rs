#[derive(serde::Serialize)]
pub struct ApiError {
    pub error: String
}

impl ApiError {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}