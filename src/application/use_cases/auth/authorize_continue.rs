use std::sync::Arc;
use actix_web::http::StatusCode;
use redis::AsyncCommands;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::spi::repository::RepositoryInterface;
use crate::dto::auth::authorize::request::AuthorizeRequest;
use crate::dto::auth::authorize::token_data::TokenData;
use crate::utils::api_response::{ApiError, ApiSuccess};

pub struct AuthorizeContinueUseCase {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthSessionRepository>,
}

impl UseCaseInterface for AuthorizeContinueUseCase {
    type Request = AuthorizeRequest;
    type Response = String;

    async fn handle(&self, data: Self::Request) -> Result<ApiSuccess<Self::Response>, ApiError> {
        let arc_data = Arc::new(data);

        if let Err(e) = self.validate_query(arc_data.clone()) {
            return Err(ApiError::new(e, StatusCode::UNPROCESSABLE_ENTITY))
        }

        let result = self.get_session_and_user_uuid(arc_data.clone());

        if let Err(e) = result {
            return Err(ApiError::new(e.0, e.1))
        }

        let (session_uuid, user_uuid) = result.unwrap();

        let mut session = match self.repository.get(session_uuid).await {
            Ok(result) => result,
            Err(e) => return Err(ApiError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
        };

        if session.user_id.is_some() {
            return Err(ApiError::new("Session already authorized".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
        }

        let token = uuid::Uuid::new_v4();

        session.user_id = Some(user_uuid);

        if let Err(e) =  self.repository.edit(session.id.unwrap(), session.clone(), vec![
            "user_id",
        ]).await {
            return Err(ApiError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
        }

        let mut conn = match self.cache.get_pool().await {
            Ok(conn) => conn,
            Err(e) => return Err(ApiError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
        };
        
        let token_data = TokenData {
            user_id: user_uuid,
            session_id: session_uuid,
        };

        let value = serde_json::to_string(&token_data).unwrap();

        if let Err(_) = conn.set_ex::<String, String, ()>(token.to_string(), value, 60 * 2)
            .await {
            return Err(ApiError::new("Failed to store authorization code".to_string(), StatusCode::INTERNAL_SERVER_ERROR))
        }

        let url = session.redirect_uri.unwrap() + "?code=" + &token.to_string() + "&state=" + &session.state.unwrap();
        Ok(ApiSuccess::new(url, StatusCode::SEE_OTHER))
    }
}

#[allow(unused)]
impl AuthorizeContinueUseCase {
    pub fn new(cache: Arc<RedisCache>, repository: Arc<OAuthSessionRepository>) -> Self {
        Self { cache, repository }
    }

    fn validate_query(&self, data: Arc<AuthorizeRequest>) -> Result<(), String> {
        if data.auth_token.is_none() {
            return Err("Missing auth token".to_string())
        }

        if data.session_id.is_none() {
            return Err("Missing session id".to_string())
        }

        if data.user_id.is_none() {
            return Err("Missing user id".to_string())
        }

        Ok(())
    }
    
    fn get_session_and_user_uuid(&self, data: Arc<AuthorizeRequest>) -> Result<(uuid::Uuid, uuid::Uuid), (String, StatusCode)>  {
        let session_id = data.session_id.as_ref().unwrap();
        let user_id = data.user_id.as_ref().unwrap();

        let session_uuid = match uuid::Uuid::parse_str(session_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(("Invalid session ID".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
        };

        let user_uuid = match uuid::Uuid::parse_str(user_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(("Invalid user ID".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
        };
        
        Ok((session_uuid, user_uuid))
    }
}