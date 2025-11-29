use std::sync::Arc;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::gateways::idp::IdpGateway;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::adapters::spi::repositories::oauth_token::OAuthTokenRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::token_authorization_code::TokenAuthorizationCodeUseCase;
use crate::application::use_cases::auth::token_refresh::TokenRefreshUseCase;
use crate::dto::auth::token::request::{TokenRefreshRequest, TokenRequest};
use crate::utils::api_response::ApiErrorResponse;

pub struct TokenController {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthSessionRepository>,
    token_repository: Arc<OAuthTokenRepository>,
    client_repository: Arc<OAuthClientRepository>,
    idp_gateway: Arc<IdpGateway>
}

impl ControllerInterface for TokenController {
    type Data = web::Either<web::Form<TokenRequest>, web::Form<TokenRefreshRequest>>;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        match data {
            web::Either::Left(e) => {
                match TokenAuthorizationCodeUseCase::new(
                    self.cache.clone(),
                    self.repository.clone(),
                    self.token_repository.clone(),
                    self.client_repository.clone(),
                    self.idp_gateway.clone(),
                ).handle(e.into_inner()).await {
                    Ok(e) => HttpResponse::Ok().json(e.data),
                    Err(e) => HttpResponse::build(StatusCode::from_u16(e.status_code).unwrap()).json(ApiErrorResponse::new(e.error)),
                }
            },
            web::Either::Right(e) => {
                match TokenRefreshUseCase::new(
                    self.repository.clone(),
                    self.token_repository.clone(),
                    self.client_repository.clone(),
                    self.idp_gateway.clone(),
                ).handle(e.into_inner()).await {
                    Ok(e) => HttpResponse::Ok().json(e.data),
                    Err(e) => HttpResponse::build(StatusCode::from_u16(e.status_code).unwrap()).json(ApiErrorResponse::new(e.error)),
                }
            }
        }
    }
}

impl TokenController {
    pub fn new(
        cache: Arc<RedisCache>,
        repository: Arc<OAuthSessionRepository>,
        token_repository: Arc<OAuthTokenRepository>,
        client_repository: Arc<OAuthClientRepository>,
        idp_gateway: Arc<IdpGateway>
    ) -> Self {
        Self { cache, repository, token_repository, client_repository, idp_gateway  }
    }
}