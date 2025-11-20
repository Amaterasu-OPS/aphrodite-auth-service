use std::sync::Arc;
use actix_web::http::StatusCode;
use deadpool_redis::redis::{AsyncCommands};
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::domain::oauth_client::OauthClient;
use crate::dto::auth::par::request::ParRequest;
use crate::dto::auth::par::response::ParResponse;
use crate::utils::api_response::{ApiError, ApiSuccess};
use crate::utils::entropy::entropy_total_bits;

pub struct ParUseCase {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthClientRepository>
}

impl UseCaseInterface for ParUseCase {
    type Request = ParRequest;
    type Response = ParResponse;

    async fn handle(&self, data: ParRequest) -> Result<ApiSuccess<Self::Response>, ApiError> {
        let arc_data = Arc::new(data);

        if let Err(e) = self.validate_state(arc_data.clone()) {
            return Err(ApiError::new(e, StatusCode::BAD_REQUEST))
        }

        let client = match self.get_client(Arc::clone(&arc_data)).await {
            Ok(e) => e,
            Err(err) => return Err(ApiError::new(format!("Getting client: {}", err), StatusCode::BAD_REQUEST))
        };

        if let Err(e) = self.validate_uris(Arc::clone(&arc_data), &client) {
            return Err(ApiError::new(format!("Error validating URIs: {}", e), StatusCode::BAD_REQUEST))
        }

        if let Err(e) = self.validate_scopes(Arc::clone(&arc_data), &client) {
            return Err(ApiError::new(format!("Error validating scopes: {}", e), StatusCode::BAD_REQUEST))
        }

        let exp = 60;
        let request_uri = String::from("urn:ietf:params:oauth:request_uri:") + &uuid::Uuid::new_v4().to_string();
        let response = ParResponse {
            request_uri: request_uri.clone(),
            expires_in: exp,
        };

        let mut conn = match self.cache.get_pool().await {
            Ok(conn) => conn,
            Err(e) => return Err(ApiError::new(format!("Getting cache connection: {}", e), StatusCode::INTERNAL_SERVER_ERROR))
        };

        let value = serde_json::to_string(&arc_data).unwrap();

        if let Err(_) = conn.set_ex::<String, String, ()>(request_uri, value, exp)
            .await{
            return Err(ApiError::new("Failed to store PAR request".to_string(), StatusCode::INTERNAL_SERVER_ERROR))
        }

        Ok(ApiSuccess::new(response, StatusCode::CREATED))
    }
}

impl ParUseCase {
    pub fn new(cache: Arc<RedisCache>, repository: Arc<OAuthClientRepository>) -> Self {
        Self { cache, repository }
    }

    async fn get_client(&self, data: Arc<ParRequest>) -> Result<OauthClient, String> {
        self.repository.get_by_slug_secret(data.client_id.clone(), data.client_secret.clone()).await
    }

    fn validate_uris(&self, data: Arc<ParRequest>, client: &OauthClient) -> Result<(), String> {
        if data.redirect_uri.is_empty() {
            return Err(String::from("Invalid redirect URI"));
        }

        if client.urls.is_none() {
            return Err(String::from("Invalid redirect URI"));
        }

        let urls = client.urls.clone().unwrap();

        if !urls.contains(&data.redirect_uri) {
            return Err(String::from("Invalid redirect URI"));
        }

        Ok(())
    }

    fn validate_scopes(&self, data: Arc<ParRequest>, client: &OauthClient) -> Result<(), String> {
        if data.scope.is_empty() {
            return Err(String::from("Invalid scopes"));
        }

        if client.scopes.is_none() {
            return Err(String::from("Invalid scopes"));
        }

        let scopes = client.scopes.clone().unwrap();
        let requested_scopes = data.scope.split(" ").collect::<Vec<&str>>();

        for scope in requested_scopes {
            if !scopes.contains(&scope.to_owned()) {
                return Err(String::from("Invalid scopes"));
            }
        }

        Ok(())
    }

    fn validate_state(&self, data: Arc<ParRequest>) -> Result<(), String> {
        if data.response_type != "code" {
            return Err(String::from("Invalid response type"));
        }

        if data.code_challenge_method != "S256" {
            return Err(String::from("Invalid code challenge method"));
        }

        if data.state.is_empty() {
            return Err(String::from("Invalid state"));
        }

        if data.code_challenge.is_empty() {
            return Err(String::from("Invalid code challenge"));
        }

        if entropy_total_bits(data.state.clone().as_str()) < 64.0 {
            return Err(String::from("Invalid state entropy"));
        }

        Ok(())
    }
}