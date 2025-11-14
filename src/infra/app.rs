use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use crate::adapters::api;
use crate::adapters::spi::db::db::DBAdapter;
use crate::adapters::spi::db::postgres_db::PostgresDB;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::spi::repository::RepositoryInterface;
use crate::infra::redis::get_redis_pool;

pub async fn start_app() -> std::io::Result<()> {
    let psql = Arc::new(DBAdapter::get_db_connection::<PostgresDB>().await.expect("Failed to connect to postgres database"));
    
    let redis_pool_data = web::Data::new(get_redis_pool());
    
    let oauth_client_repository = web::Data::new(OAuthClientRepository::new(String::from("oauth_client"), psql));

    HttpServer::new(move || {
        App::new()
        .service(web::scope("/api/v1")
            .service(api::health::router::health_router())
            .service(api::auth::router::auth_router())
        )
        .app_data(redis_pool_data.clone())
        .app_data(oauth_client_repository.clone())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}