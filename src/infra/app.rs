use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use crate::adapters::api;
use crate::adapters::spi::cache::cache::CacheAdapter;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::db::db::DBAdapter;
use crate::adapters::spi::db::postgres_db::PostgresDB;
use crate::infra::dependencies::add_dependencies;

pub async fn start_app() -> std::io::Result<()> {
    let psql = Arc::new(DBAdapter::get_db_connection::<PostgresDB>().await.expect("Failed to connect to postgres database"));
    let redis = Arc::new(CacheAdapter::get_cache_connection::<RedisCache>().expect("Failed to connect to redis"));

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3001");
        
        App::new()
        .wrap(Logger::default())
        .wrap(cors)
        .service(web::scope("/api/v1")
            .service(api::health::router::health_router())
            .service(api::auth::router::auth_router())
        )
        .configure(|config| add_dependencies(
            config,
            psql.clone(),
            redis.clone()
        ))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}