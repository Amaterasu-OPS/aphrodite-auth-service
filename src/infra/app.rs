use actix_web::{App, HttpServer, web};
use deadpool_redis::Config;

use crate::adapters::api;

pub async fn start_app() -> std::io::Result<()> {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
    let cfg = Config::from_url(redis_url);
    let redis_pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).expect("Cannot create Redis pool");

    let redis_pool_data = web::Data::new(redis_pool);

    HttpServer::new(move || {
        App::new()
        .service(web::scope("/api/v1")
            .service(api::health::router::health_router())
            .service(api::auth::router::auth_router())
        )
        .app_data(redis_pool_data.clone())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}