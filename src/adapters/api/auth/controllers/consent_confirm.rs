use std::sync::Arc;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_consent::OAuthConsentRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::consent_confirm::ConsentConfirmUseCase;
use crate::dto::auth::consent_confirm::request::ConsentConfirmRequest;
use crate::utils::api_response::ApiErrorResponse;

pub struct ConsentConfirmController {
    repository: Arc<OAuthConsentRepository>,
    session_repository: Arc<OAuthSessionRepository>,
    client_repository: Arc<OAuthClientRepository>,
}

impl ControllerInterface for ConsentConfirmController {
    type Data = ConsentConfirmRequest;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        match ConsentConfirmUseCase::new(
            self.repository.clone(),
            self.session_repository.clone(),
            self.client_repository.clone(),
        ).handle(data).await {
            Ok(e) => HttpResponse::Ok().json(e.data),
            Err(e) => HttpResponse::build(StatusCode::from_u16(e.status_code).unwrap()).json(ApiErrorResponse::new(e.error)),
        }
    }
}

impl ConsentConfirmController {
    pub fn new(repository: Arc<OAuthConsentRepository>, session_repository: Arc<OAuthSessionRepository>, client_repository: Arc<OAuthClientRepository>) -> Self {
        Self { repository, session_repository, client_repository }
    }
}