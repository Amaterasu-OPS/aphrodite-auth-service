use actix_web::http::StatusCode;
use serde::Serialize;
use crate::domain::idp::{IdPIdTokenRequest, IdPIdTokenResponse, IdpUser, IdpVerifyCredentialRequest, IdpVerifyCredentialResponse};
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

    async fn post<T: Serialize>(&self, url: String, body: T) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();

        client.post(format!("{}{}", self.base_url, url))
            .json(&serde_json::json!(body))
            .header("X-api-key", std::env::var("IDP_API_KEY").unwrap_or(String::new()))
            .send()
            .await
    }

    async fn get(&self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();

        client.get(format!("{}{}", self.base_url, url))
            .header("X-api-key", std::env::var("IDP_API_KEY").unwrap_or(String::new()))
            .send()
            .await
    }

    pub async fn get_id_token_v1(&self, body: IdPIdTokenRequest) -> Result<String, ApiError> {
        let resp = match self.post("/api/v1/credentials/id_token".to_string(), body).await {
            Ok(resp) => resp,
            Err(e) => return Err(ApiError::new(format!("Failed to send request to IDP: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        let result = match resp.json::<IdPIdTokenResponse>().await {
            Ok(result) => result,
            Err(e) => return Err(ApiError::new(format!("Failed to parse response from IDP: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        Ok(result.id_token)
    }

    pub async fn verify_auth_token_v1(&self, body: IdpVerifyCredentialRequest) -> Result<bool, ApiError> {
        let resp = match self.post("/api/v1/user/verify-credential".to_string(), body).await {
            Ok(resp) => resp,
            Err(e) => return Err(ApiError::new(format!("Failed to send request to IDP: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        let result = match resp.json::<IdpVerifyCredentialResponse>().await {
            Ok(result) => result,
            Err(e) => return Err(ApiError::new(format!("Failed to parse response from IDP: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        Ok(result.verified)
    }

    pub async fn get_user_by_id_v1(&self, id: uuid::Uuid) -> Result<IdpUser, ApiError> {
        let resp = match self.get(format!("/api/v1/user/{}", id)).await {
            Ok(resp) => resp,
            Err(e) => return Err(ApiError::new(format!("Failed to send request to IDP: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        let result = match resp.json::<IdpUser>().await {
            Ok(result) => result,
            Err(e) => return Err(ApiError::new(format!("Failed to parse response from IDP: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        Ok(result)
    }
}