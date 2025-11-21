use std::sync::Arc;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::adapters::spi::repositories::oauth_token::OAuthTokenRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::token_authorization_code::TokenAuthorizationCodeUseCase;
use crate::dto::auth::token::request::TokenRequest;
use crate::utils::api_response::ApiErrorResponse;

pub struct TokenController {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthSessionRepository>,
    token_repository: Arc<OAuthTokenRepository>
}

impl ControllerInterface for TokenController {
    type Data = TokenRequest;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        match TokenAuthorizationCodeUseCase::new(
            self.cache.clone(),
            self.repository.clone(),
            self.token_repository.clone()
        ).handle(data).await {
            Ok(e) => HttpResponse::Ok().json(e.data),
            Err(e) => HttpResponse::build(StatusCode::from_u16(e.status_code).unwrap()).json(ApiErrorResponse::new(e.error)),
        }
    }
}

impl TokenController {
    pub fn new(cache: Arc<RedisCache>, repository: Arc<OAuthSessionRepository>, token_repository: Arc<OAuthTokenRepository>) -> Self {
        Self { cache, repository, token_repository  }
    }
}