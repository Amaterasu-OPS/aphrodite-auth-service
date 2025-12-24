use actix_web::{Responder, Scope, get, post, web, HttpRequest, HttpResponse};
use crate::adapters::api::auth::controllers::authorize::AuthorizeController;
use crate::adapters::api::auth::controllers::consent_confirm::ConsentConfirmController;
use crate::adapters::api::auth::controllers::consent_info::ConsentInfoController;
use crate::adapters::api::auth::controllers::par::ParController;
use crate::adapters::api::auth::controllers::token::TokenController;
use crate::adapters::api::auth::controllers::userinfo::UserinfoController;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::gateways::idp::IdpGateway;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_consent::OAuthConsentRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::adapters::spi::repositories::oauth_token::OAuthTokenRepository;
use crate::application::api::controller::ControllerInterface;
use crate::dto::auth::authorize::request::AuthorizeRequest;
use crate::dto::auth::consent_confirm::request::ConsentConfirmRequest;
use crate::dto::auth::consent_info::request::ConsentInfoRequest;
use crate::dto::auth::par::{request::ParRequest};
use crate::dto::auth::token::request::{TokenRefreshRequest, TokenRequest};
use crate::dto::auth::userinfo::request::UserinfoRequest;

pub fn auth_router() -> Scope {
    web::scope("/auth")
        .service(par_handler)
        .service(authorize_handler)
        .service(token_handler)
        .service(consent_info_handler)
        .service(consent_confirm_handler)
        .service(userinfo_handler)
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
    consent_repository: web::Data<OAuthConsentRepository>,
    idp_gateway: web::Data<IdpGateway>,
) -> impl Responder {
    AuthorizeController::new(
        cache.into_inner(),
        repository.into_inner(),
        consent_repository.into_inner(),
        idp_gateway.into_inner(),
    ).handle(data.into_inner()).await
}

#[post("/token")]
async fn token_handler(
    data: web::Either<web::Form<TokenRequest>, web::Form<TokenRefreshRequest>>,
    cache: web::Data<RedisCache>,
    repository: web::Data<OAuthSessionRepository>,
    token_repository: web::Data<OAuthTokenRepository>,
    client_repository: web::Data<OAuthClientRepository>,
    idp_gateway: web::Data<IdpGateway>,
) -> impl Responder {
    TokenController::new(
        cache.into_inner(),
        repository.into_inner(),
        token_repository.into_inner(),
        client_repository.into_inner(),
        idp_gateway.into_inner(),
    ).handle(data).await
}

#[get("/consent/info")]
async fn consent_info_handler(
    data: web::Query<ConsentInfoRequest>,
    repository: web::Data<OAuthClientRepository>,
    session_repository: web::Data<OAuthSessionRepository>,
) -> impl Responder {
    ConsentInfoController::new(
        repository.into_inner(),
        session_repository.into_inner(),
    ).handle(data.into_inner()).await
}

#[post("/consent/confirm")]
async fn consent_confirm_handler(
    data: web::Json<ConsentConfirmRequest>,
    repository: web::Data<OAuthConsentRepository>,
    client_repository: web::Data<OAuthClientRepository>,
    session_repository: web::Data<OAuthSessionRepository>,
) -> impl Responder {
    ConsentConfirmController::new(
        repository.into_inner(),
        session_repository.into_inner(),
        client_repository.into_inner()
    ).handle(data.into_inner()).await
}

#[post("/userinfo")]
async fn userinfo_handler(
    req: HttpRequest,
    data: web::Json<UserinfoRequest>,
    repository: web::Data<OAuthSessionRepository>,
    token_repository: web::Data<OAuthTokenRepository>,
    idp_gateway: web::Data<IdpGateway>,
    cache: web::Data<RedisCache>,
) -> impl Responder {
    const TOKEN_HEADER: &str = "x-access-token";
    
    if req.headers().get(TOKEN_HEADER).is_none() {
        return HttpResponse::BadRequest().body(format!("Missing {} header", TOKEN_HEADER));
    }
    
    let header = req.headers().get(TOKEN_HEADER).unwrap().to_str().ok().unwrap();
    
    UserinfoController::new(
        cache.into_inner(),
        repository.into_inner(),
        token_repository.into_inner(),
        idp_gateway.into_inner(),
        header.to_string()
    ).handle(data.into_inner()).await
}