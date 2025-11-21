use std::sync::Arc;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::authorize::AuthorizeUseCase;
use crate::application::use_cases::auth::authorize_continue::AuthorizeContinueUseCase;
use crate::dto::auth::authorize::request::AuthorizeRequest;
use crate::utils::api_response::{ApiError, ApiErrorResponse, ApiSuccess};

pub struct AuthorizeController {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthSessionRepository>,
}

impl ControllerInterface for AuthorizeController {
    type Data = AuthorizeRequest;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        if !data.auth_token.is_none()  {
            return self.format_result(AuthorizeContinueUseCase::new(
                self.cache.clone(),
                self.repository.clone(),
            ).handle(data).await);
        }

        self.format_result(AuthorizeUseCase::new(
            self.cache.clone(),
            self.repository.clone()
        ).handle(data).await)
    }
}

impl AuthorizeController {
    pub fn new(
        cache: Arc<RedisCache>,
        repository: Arc<OAuthSessionRepository>,
    ) -> Self {
        Self {
            cache,
            repository,
        }
    }
    fn format_result(&self, result: Result<ApiSuccess<String>, ApiError>) -> HttpResponse {
        match result {
            Ok(e) => HttpResponse::SeeOther().append_header(("Location", e.data)).finish(),
            Err(e) => HttpResponse::build(
                StatusCode::from_u16(e.status_code).unwrap()
            ).json(ApiErrorResponse::new(e.error))
        }
    }
}