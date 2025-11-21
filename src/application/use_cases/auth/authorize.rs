use std::sync::Arc;
use actix_web::http::StatusCode;
use redis::AsyncCommands;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::spi::repository::RepositoryInterface;
use crate::domain::oauth_session::OauthSession;
use crate::dto::auth::authorize::request::AuthorizeRequest;
use crate::dto::auth::par::request::ParRequest;
use crate::utils::api_response::{ApiError, ApiSuccess};

pub struct AuthorizeUseCase {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthSessionRepository>,
}

impl UseCaseInterface for AuthorizeUseCase {
    type Request = AuthorizeRequest;
    type Response = String;

    async fn handle(&self, data: Self::Request) -> Result<ApiSuccess<Self::Response>, ApiError> {
        let request = match self.get_request_from_par_uri(&data).await {
            Ok(e) => e,
            Err(e) => {
                return Err(ApiError::new(e.0, e.1));
            }
        };

        let requested_scopes = request.scope.split(" ").collect::<Vec<&str>>().iter().map(|e| e.to_string()).collect::<Vec<String>>();

        let result = match self.repository.insert(OauthSession {
            id: None,
            client_id: Some(data.client_id.unwrap()),
            response_type: Some(request.response_type),
            code_challenge_method: Some(request.code_challenge_method),
            status: None,
            consent_granted_at: None,
            created_at: None,
            code_challenge: Some(request.code_challenge),
            state: Some(request.state),
            redirect_uri: Some(request.redirect_uri),
            scopes: Some(requested_scopes),
            user_id: None,
            updated_at: None,
        }).await {
            Ok(e) => e,
            Err(_) => {
                return Err(ApiError::new("Failed to create OAuth session".to_string(), StatusCode::INTERNAL_SERVER_ERROR));
            }
        };

        let url = std::env::var("LOGIN_PAGE_URL").unwrap_or("http://localhost:3001/".to_string()) + "?session_id=" + result.id.unwrap().to_string().as_str();
        Ok(ApiSuccess::new(url, StatusCode::SEE_OTHER))
    }
}

impl AuthorizeUseCase {
    pub fn new(cache: Arc<RedisCache>, repository: Arc<OAuthSessionRepository>) -> Self {
        Self { cache, repository }
    }

    async fn get_request_from_par_uri(&self, data: &AuthorizeRequest) -> Result<ParRequest, (String, StatusCode)> {
        let uri = data.uri.as_ref().unwrap();
        let client_id = data.client_id.as_ref().unwrap();

        if !uri.starts_with("urn:ietf:params:oauth:request_uri:") {
            return Err(("Invalid URI".to_string(), StatusCode::BAD_REQUEST))
        }

        let mut conn = match self.cache.get_pool().await {
            Ok(conn) => conn,
            Err(e) => {
                return Err((e, StatusCode::INTERNAL_SERVER_ERROR))
            }
        };

        let value = match conn.get::<String, String>(uri.clone()).await {
            Ok(value) => value,
            Err(_) => {
                return Err(("URI not found".to_string(), StatusCode::BAD_REQUEST))
            }
        };

        let request = serde_json::from_str::<ParRequest>(&value).unwrap();

        if request.client_id != *client_id {
            return Err(("Invalid client id".to_string(), StatusCode::BAD_REQUEST))
        }

        if let Err(_) = conn.del::<String, String>(uri.clone()).await {
            return Err(("Failed to delete URI".to_string(), StatusCode::BAD_REQUEST))
        }

        Ok(request)
    }
}