use std::sync::Arc;
use actix_web::http::StatusCode;
use redis::AsyncCommands;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::utils::api_response::{ApiError, ApiSuccess};
use crate::adapters::spi::gateways::idp::IdpGateway;
use crate::adapters::spi::repositories::oauth_token::OAuthTokenRepository;
use crate::application::spi::repository::RepositoryInterface;
use crate::dto::auth::userinfo::request::UserinfoRequest;
use crate::dto::auth::userinfo::response::UserinfoResponse;
use crate::utils::hasher::hash_sha256;

pub struct UserinfoUseCase {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthSessionRepository>,
    token_repository: Arc<OAuthTokenRepository>,
    idp_gateway: Arc<IdpGateway>,
    access_token: String
}

impl UseCaseInterface for UserinfoUseCase {
    type Request = UserinfoRequest;
    type Response = UserinfoResponse;

    async fn handle(&self, data: Self::Request) -> Result<ApiSuccess<Self::Response>, ApiError> {
        let Ok(mut conn) = self.cache.get_pool().await else {
            return Err(ApiError::new(String::from("Getting cache connection"), StatusCode::INTERNAL_SERVER_ERROR))
        };

        let Ok(value) = conn.get::<String, Option<String>>(data.sub.clone()).await else  {
            return Err(ApiError::new(String::from("Invalid authorization code"), StatusCode::BAD_REQUEST));
        };

        if value.is_some() {
            let Ok(result) = serde_json::from_str::<Self::Response>(&value.unwrap()) else {
                return Err(ApiError::new(String::from("Failed to parse session data"), StatusCode::INTERNAL_SERVER_ERROR));
            };

            return Ok(ApiSuccess::new(result, StatusCode::OK))
        };

        let Ok(token) = self.token_repository.get_by_access_token(hash_sha256(self.access_token.clone().as_str())).await else {
            return Err(ApiError::new(String::from("Invalid access token"), StatusCode::BAD_REQUEST));
        };

        let Ok(session) = self.repository.get(token.session_id.unwrap()).await else {
            return Err(ApiError::new(String::from("Invalid session"), StatusCode::BAD_REQUEST));
        };

        let Ok(user) = self.idp_gateway.get_user_by_id_v1(session.user_id.clone().unwrap()).await else {
            return Err(ApiError::new(String::from("Failed to get user"), StatusCode::INTERNAL_SERVER_ERROR));
        };

        let result = UserinfoResponse {
            sub: user.id,
            given_name: user.name.unwrap_or_default(),
            family_name: user.family_name.unwrap_or_default(),
            gender: user.gender.unwrap_or_default(),
            email: user.email.unwrap_or_default(),
            created_at: user.created_at.unwrap_or_default(),
        };

        let string_result = serde_json::to_string(&result).unwrap();

        if let Err(_) = conn.set_ex::<String, String, ()>(format!("sub:{}", data.sub.clone()), string_result, 60 * 5).await {
            return Err(ApiError::new("Failed to store sub".to_string(), StatusCode::INTERNAL_SERVER_ERROR))
        }

        Ok(ApiSuccess::new(result, StatusCode::OK))
    }
}

impl UserinfoUseCase {
    pub fn new(
        cache: Arc<RedisCache>,
        repository: Arc<OAuthSessionRepository>,
        token_repository: Arc<OAuthTokenRepository>,
        idp_gateway: Arc<IdpGateway>,
        access_token: String
    ) -> Self {
        Self {
            cache,
            repository,
            token_repository,
            idp_gateway,
            access_token
        }
    }
}