use actix_web::http::StatusCode;
use crate::domain::idp_id_token::{IdPIdToken, IdPIdTokenPayload};
use crate::utils::api_response::ApiError;

pub struct IdpGateway {
    base_url: String
}

impl IdpGateway {
    pub fn new() -> Self {

        Self {
            base_url: std::env::var("IDP_URL").expect("IDP_BASE_URL not found")
        }
    }

    pub async fn get_id_token_v1(&self, body: IdPIdTokenPayload) -> Result<String, ApiError> {
        let client = reqwest::Client::new();

        let resp = match client.post(format!("{}/api/v1/credentials/id_token", self.base_url))
            .json(&serde_json::json!(body))
            .header("X-api-key", std::env::var("IDP_API_KEY").unwrap_or(String::new()))
            .send()
            .await {
            Ok(resp) => resp,
            Err(e) => return Err(ApiError::new(format!("Failed to send request to IDP: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        let body = match resp.json::<IdPIdToken>().await {
            Ok(body) => body,
            Err(e) => return Err(ApiError::new(format!("Failed to parse response from IDP: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        Ok(body.id_token)
    }
}