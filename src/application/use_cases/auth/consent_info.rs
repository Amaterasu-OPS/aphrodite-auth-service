use std::sync::Arc;
use actix_web::http::StatusCode;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::spi::repository::RepositoryInterface;
use crate::dto::auth::consent_info::request::ConsentInfoRequest;
use crate::dto::auth::consent_info::response::ConsentInfoResponse;
use crate::utils::api_response::{ApiError, ApiSuccess};

pub struct ConsentInfoUseCase {
    repository: Arc<OAuthClientRepository>,
    session_repository: Arc<OAuthSessionRepository>
}

impl UseCaseInterface for ConsentInfoUseCase {
    type Request = ConsentInfoRequest;
    type Response = ConsentInfoResponse;

    async fn handle(&self, data: Self::Request) -> Result<ApiSuccess<Self::Response>, ApiError> {
        let Ok(session) = self.session_repository.get(data.session_id).await else {
            return Err(ApiError::new(String::from("Session not found"), StatusCode::BAD_REQUEST))
        };

        if session.consent_granted_at.is_some() {
            return Err(ApiError::new(String::from("Consent already granted for this session"), StatusCode::BAD_REQUEST));
        }
        
        let Ok(client) = self.repository.get_by_slug(session.client_id.clone().unwrap()).await else {
            return Err(ApiError::new(String::from("Client not found"), StatusCode::BAD_REQUEST))
        };
        
        Ok(ApiSuccess::new(ConsentInfoResponse {
            client_id: client.slug.unwrap(),
            scopes: client.scopes.unwrap(),
            name: client.name.unwrap(),
            mandatory_scopes: client.mandatory_scopes.unwrap(),
            logos: client.logos.unwrap().to_vec(),
            created_at: client.created_at.unwrap(),
        }, StatusCode::OK))
    }
}

impl ConsentInfoUseCase {
    pub fn new(repository: Arc<OAuthClientRepository>, session_repository: Arc<OAuthSessionRepository>) -> Self {
        Self { repository, session_repository }
    }
}

