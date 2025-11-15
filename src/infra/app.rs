use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use crate::adapters::api;
use crate::adapters::spi::cache::cache::CacheAdapter;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::db::db::DBAdapter;
use crate::adapters::spi::db::postgres_db::PostgresDB;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::spi::repository::RepositoryInterface;

pub async fn start_app() -> std::io::Result<()> {
    let psql = Arc::new(DBAdapter::get_db_connection::<PostgresDB>().await.expect("Failed to connect to postgres database"));
    let redis = CacheAdapter::get_cache_connection::<RedisCache>().expect("Failed to connect to redis");
    
    let oauth_client_repository = web::Data::new(OAuthClientRepository::new(String::from("oauth_client"), psql));
    let redis_cache = web::Data::new(redis);

    HttpServer::new(move || {
        App::new()
        .service(web::scope("/api/v1")
            .service(api::health::router::health_router())
            .service(api::auth::router::auth_router())
        )
        .app_data(redis_cache.clone())
        .app_data(oauth_client_repository.clone())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}