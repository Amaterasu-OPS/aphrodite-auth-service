use std::sync::Arc;
use actix_web::http::StatusCode;
use redis::AsyncCommands;
use uuid::Uuid;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_consent::OAuthConsentRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::spi::repository::RepositoryInterface;
use crate::domain::oauth_session::OauthSession;
use crate::dto::auth::authorize::request::AuthorizeRequest;
use crate::dto::auth::authorize::token_data::TokenData;
use crate::utils::api_response::{ApiError, ApiSuccess};

pub struct AuthorizeContinueUseCase {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthSessionRepository>,
    consent_repository: Arc<OAuthConsentRepository>,
}

impl UseCaseInterface for AuthorizeContinueUseCase {
    type Request = AuthorizeRequest;
    type Response = String;

    async fn handle(&self, data: Self::Request) -> Result<ApiSuccess<Self::Response>, ApiError> {
        let arc_data = Arc::new(data);

        if let Err(e) = self.validate_query(arc_data.clone()).await {
            return Err(ApiError::new(e, StatusCode::UNPROCESSABLE_ENTITY))
        }

        let result = self.get_session_and_user_uuid(arc_data.clone());

        if let Err(e) = result {
            return Err(ApiError::new(e.0, e.1))
        }

        let (session_uuid, user_uuid) = result.unwrap();

        let mut session = self
            .repository
            .get(session_uuid)
            .await
            .map_err(|e| ApiError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

        let token = Uuid::new_v4();

        if arc_data.consent_id.is_some() {
            if let Err(e) = self.validate_consent(&mut session, arc_data.clone()).await {
                return Err(e)
            }
        } else {
            if let Err(e) = self.save_user_and_consent(&mut session, user_uuid).await {
                if e.status_code == StatusCode::SEE_OTHER {
                    return Ok(ApiSuccess::new(e.error, StatusCode::SEE_OTHER))
                }

                return Err(e)
            }
        }

        let mut conn = self
            .cache
            .get_pool()
            .await
            .map_err(|e| ApiError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        
        let token_data = TokenData {
            user_id: user_uuid,
            session_id: session_uuid,
        };

        let value = serde_json::to_string(&token_data).unwrap();

        if let Err(_) = conn.set_ex::<String, String, ()>(token.to_string(), value, 60 * 2).await {
            return Err(ApiError::new("Failed to store authorization code".to_string(), StatusCode::INTERNAL_SERVER_ERROR))
        }

        let url = format!("{}?code={}&state={}", session.redirect_uri.unwrap(), token.to_string(), session.state.unwrap());
        Ok(ApiSuccess::new(url, StatusCode::SEE_OTHER))
    }
}

#[allow(unused)]
impl AuthorizeContinueUseCase {
    pub fn new(cache: Arc<RedisCache>, repository: Arc<OAuthSessionRepository>, consent_repository: Arc<OAuthConsentRepository>) -> Self {
        Self { cache, repository, consent_repository }
    }

    async fn validate_query(&self, data: Arc<AuthorizeRequest>) -> Result<(), String> {
        if data.auth_token.is_none() && data.consent_id.is_none() {
            return Err("Missing auth token or consent id".to_string())
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

        let session_uuid = match Uuid::parse_str(session_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(("Invalid session ID".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
        };

        Ok((session_uuid, data.user_id.unwrap()))
    }

    async fn check_is_consent_is_granted(&self, client_id: String, user_id: Uuid) -> Result<bool, ApiError> {
        if let Err(e) = self.consent_repository.get_by_client_and_user_id(
            client_id.clone(),
            user_id.clone()
        ).await {
            if e == String::from("Consent not found") {
                return Ok(false)
            }

            return Err(ApiError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
        }

        Ok(true)
    }

    async fn save_user_and_consent(&self, session: &mut OauthSession, user_uuid: Uuid) -> Result<(), ApiError> {
        if session.user_id.is_some() {
            return Err(ApiError::new("User already set for this session".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
        }

        session.user_id = Some(user_uuid);

        if let Err(e) =  self.repository.edit(session.id.unwrap(), session.clone(), vec![
            "user_id",
        ]).await {
            return Err(ApiError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
        }

        if let Ok(e) = self.check_is_consent_is_granted(session.client_id.clone().unwrap(), user_uuid).await {
            if e {
                session.consent_granted_at = Some(chrono::Utc::now().naive_utc());
            } else {
                let url = std::env::var("CONSENT_PAGE_URL").unwrap_or_default();
                return Err(ApiError::new(String::from(url), StatusCode::SEE_OTHER))
            }
        } else {
            return Err(ApiError::new("Failed to check consent".to_string(), StatusCode::INTERNAL_SERVER_ERROR))
        }
        
        Ok(())
    }

    async fn validate_consent(&self, session: &mut OauthSession, data: Arc<AuthorizeRequest>) -> Result<(), ApiError> {
        if let Ok(e) = self.consent_repository.get(data.consent_id.unwrap()).await {
            if e.user_id.unwrap() != session.user_id.clone().unwrap() || e.client_id.unwrap() != session.client_id.clone().unwrap() {
                return Err(ApiError::new("Invalid consent ID".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
            }
        }

        Ok(())
    }
}