use std::sync::Arc;
use actix_web::web;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::db::postgres_db::PostgresDB;
use crate::adapters::spi::gateways::idp::IdpGateway;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_consent::OAuthConsentRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::adapters::spi::repositories::oauth_token::OAuthTokenRepository;
use crate::application::spi::repository::RepositoryInterface;

pub fn add_dependencies(config: &mut web::ServiceConfig, psql: Arc<PostgresDB>, redis: Arc<RedisCache>) {
    let oauth_client_repository = web::Data::new(OAuthClientRepository::new(String::from("oauth_client"), psql.clone()));
    let oauth_session_repository = web::Data::new(OAuthSessionRepository::new(String::from("oauth_session"), psql.clone()));
    let oauth_token_repository = web::Data::new(OAuthTokenRepository::new(String::from("oauth_token"), psql.clone()));
    let oauth_consent_repository = web::Data::new(OAuthConsentRepository::new(String::from("oauth_consent"), psql.clone()));

    let idp_gateway = web::Data::new(IdpGateway::new());

    let redis_cache = web::Data::new(redis.as_ref().to_owned());

    config.app_data(oauth_client_repository.clone());
    config.app_data(oauth_session_repository.clone());
    config.app_data(oauth_token_repository.clone());
    config.app_data(oauth_consent_repository.clone());

    config.app_data(redis_cache.clone());

    config.app_data(idp_gateway.clone());
}