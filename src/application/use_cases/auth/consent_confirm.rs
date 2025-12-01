use std::sync::Arc;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_consent::OAuthConsentRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::spi::repository::RepositoryInterface;
use crate::domain::oauth_consent::OauthConsent;
use crate::dto::auth::consent_confirm::request::ConsentConfirmRequest;
use crate::dto::auth::consent_confirm::response::ConsentConfirmResponse;
use crate::utils::api_response::{ApiError, ApiSuccess};

pub struct ConsentConfirmUseCase {
    repository: Arc<OAuthConsentRepository>,
    session_repository: Arc<OAuthSessionRepository>,
    client_repository: Arc<OAuthClientRepository>,
}

impl UseCaseInterface for ConsentConfirmUseCase {
    type Request = ConsentConfirmRequest;
    type Response = ConsentConfirmResponse;

    async fn handle(&self, data: Self::Request) -> Result<ApiSuccess<Self::Response>, ApiError> {
        let Ok(mut session) = self.session_repository.get(data.session_id).await else {
            return Err(ApiError::new(String::from("Session not found"), actix_web::http::StatusCode::BAD_REQUEST))
        };

        if session.consent_granted_at.is_some() {
            return Err(ApiError::new(String::from("Consent already granted for this session"), actix_web::http::StatusCode::BAD_REQUEST));
        }

        let Ok(client) = self.client_repository.get_by_slug(session.client_id.clone().unwrap()).await else {
            return Err(ApiError::new(String::from("Client not found"), actix_web::http::StatusCode::BAD_REQUEST))
        };
        
        if let Err(e) = self.check_scopes(
            client.scopes.clone().unwrap_or_default(),
            client.mandatory_scopes.clone().unwrap_or_default(),
            data.scopes.clone()
        ) {
            return Err(e);
        }
        
        if session.user_id.is_none() || session.client_id.is_none() {
            return Err(ApiError::new(String::from("Session is not authenticated"), actix_web::http::StatusCode::BAD_REQUEST));
        }

        let Ok(consent) = self.repository.insert(OauthConsent{
            id: None,
            scopes: Some(data.scopes.clone()),
            status: None,
            created_at: None,
            client_id: Some(session.client_id.clone().unwrap()),
            user_id: Some(session.user_id.clone().unwrap()),
            updated_at: None,
        }).await else {
            return Err(ApiError::new(String::from("Failed to create consent"), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR));
        };

        session.consent_granted_at = Some(chrono::Utc::now().naive_utc());
        session.scopes = Some(data.scopes.clone());

        if let Err(e) = self.session_repository.edit(session.id.unwrap(), session.clone(), vec!["consent_granted_at", "scopes"]).await {
            return Err(ApiError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        Ok(ApiSuccess::new(ConsentConfirmResponse {
            redirect_url: format!(
                "/api/v1/auth/authorize?user_id={}&session_id={}&consent_id={}",
                session.user_id.clone().unwrap(),
                session.id.clone().unwrap(),
                consent.id.clone().unwrap()
            )
        }, actix_web::http::StatusCode::OK))
    }
}

impl ConsentConfirmUseCase {
    pub fn new(repository: Arc<OAuthConsentRepository>, session_repository: Arc<OAuthSessionRepository>, client_repository: Arc<OAuthClientRepository>) -> Self {
        Self { repository, session_repository, client_repository }
    }

    fn check_scopes(&self, scopes: Vec<String>, mandatory_scopes: Vec<String>, data_scopes: Vec<String>) -> Result<(), ApiError> {
        for scope in data_scopes.iter() {
            if !scopes.contains(scope) {
                return Err(ApiError::new(format!("Scope '{}' is not allowed for this client", scope), actix_web::http::StatusCode::BAD_REQUEST));
            }
        }

        for scope in mandatory_scopes.iter() {
            if !data_scopes.contains(scope) {
                return Err(ApiError::new(format!("Mandatory scope '{}' is missing", scope), actix_web::http::StatusCode::BAD_REQUEST));
            }
        }

        Ok(())
    }
}

