use actix_web::{Responder, Scope, get, post, web};
use crate::adapters::api::auth::controllers::authorize::AuthorizeController;
use crate::adapters::api::auth::controllers::par::ParController;
use crate::adapters::api::auth::controllers::token::TokenController;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::adapters::spi::repositories::oauth_token::OAuthTokenRepository;
use crate::application::api::controller::ControllerInterface;
use crate::dto::auth::authorize::request::AuthorizeRequest;
use crate::dto::auth::par::{request::ParRequest};
use crate::dto::auth::token::request::TokenRequest;

pub fn auth_router() -> Scope {
    web::scope("/auth")
        .service(par_handler)
        .service(authorize_handler)
        .service(token_handler)
}

#[post("/par")]
async fn par_handler(
    data: web::Form<ParRequest>,
    cache: web::Data<RedisCache>,
    repository: web::Data<OAuthClientRepository>,
) -> impl Responder {
    ParController::new(
        cache.into_inner(),
        repository.into_inner(),
    ).handle(data.into_inner()).await
}

#[get("/authorize")]
async fn authorize_handler(
    data: web::Query<AuthorizeRequest>,
    cache: web::Data<RedisCache>,
    repository: web::Data<OAuthSessionRepository>,
) -> impl Responder {
    AuthorizeController::new(
        cache.into_inner(),
        repository.into_inner(),
    ).handle(data.into_inner()).await
}

#[post("/token")]
async fn token_handler(
    data: web::Query<TokenRequest>,
    cache: web::Data<RedisCache>,
    repository: web::Data<OAuthSessionRepository>,
    token_repository: web::Data<OAuthTokenRepository>,
) -> impl Responder {
    TokenController::new(
        cache.into_inner(),
        repository.into_inner(),
        token_repository.into_inner(),
    ).handle(data.into_inner()).await
}