use std::sync::Arc;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::gateways::idp::IdpGateway;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::adapters::spi::repositories::oauth_token::OAuthTokenRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::userinfo::UserinfoUseCase;
use crate::dto::auth::userinfo::request::UserinfoRequest;
use crate::utils::api_response::ApiErrorResponse;

pub struct UserinfoController {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthSessionRepository>,
    token_repository: Arc<OAuthTokenRepository>,
    idp_gateway: Arc<IdpGateway>,
    access_token: String
}

impl ControllerInterface for UserinfoController {
    type Data = UserinfoRequest;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        match UserinfoUseCase::new(
            self.cache.clone(),
            self.repository.clone(),
            self.token_repository.clone(),
            self.idp_gateway.clone(),
            self.access_token.clone()
        ).handle(data).await {
            Ok(e) => HttpResponse::Ok().json(e.data),
            Err(e) => HttpResponse::build(StatusCode::from_u16(e.status_code).unwrap()).json(ApiErrorResponse::new(e.error)),
        }
    }
}

impl UserinfoController {
    pub fn new(
        cache: Arc<RedisCache>,
        repository: Arc<OAuthSessionRepository>,
        token_repository: Arc<OAuthTokenRepository>,
        idp_gateway: Arc<IdpGateway>,
        access_token: String
    ) -> Self {
        Self { cache, repository, token_repository, idp_gateway, access_token  }
    }
}