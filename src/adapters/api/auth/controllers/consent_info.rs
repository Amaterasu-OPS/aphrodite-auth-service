use std::sync::Arc;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::consent_info::ConsentInfoUseCase;
use crate::dto::auth::consent_info::request::ConsentInfoRequest;
use crate::utils::api_response::ApiErrorResponse;

pub struct ConsentInfoController {
    repository: Arc<OAuthClientRepository>,
    session_repository: Arc<OAuthSessionRepository>,
}

impl ControllerInterface for ConsentInfoController {
    type Data = ConsentInfoRequest;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        match ConsentInfoUseCase::new(
            self.repository.clone(),
            self.session_repository.clone()
        ).handle(data).await {
            Ok(e) => HttpResponse::Ok().json(e.data),
            Err(e) => HttpResponse::build(StatusCode::from_u16(e.status_code).unwrap()).json(ApiErrorResponse::new(e.error)),
        }
    }
}

impl ConsentInfoController {
    pub fn new(repository: Arc<OAuthClientRepository>, session_repository: Arc<OAuthSessionRepository>) -> Self {
        Self { repository, session_repository }
    }
}