use actix_web::{HttpResponse, Responder, Scope, get, post, web};
use deadpool_redis::redis::AsyncCommands;

use crate::dto::auth::par::{request::ParRequest, response::ParResponse};

pub fn auth_router() -> Scope {
    web::scope("/auth")
        .service(par_handler)
        .service(authorize_handler)
        .service(token_handler)
}

#[post("/par")]
async fn par_handler(
    data: web::Json<ParRequest>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> impl Responder {
    let exp = 30;
    let request_uri = String::from("urn:ietf:params:oauth:request_uri:") + &uuid::Uuid::new_v4().to_string();
    let response = ParResponse {
        request_uri: request_uri.clone(),
        expires_in: exp,
    };

    let mut conn = redis_pool.get()
        .await
        .expect("Cannot get Redis connection from pool");

    let value = serde_json::to_string(&data.into_inner()).unwrap();

    conn.set_ex::<String, String, ()>(request_uri, value, exp)
        .await
        .expect("Failed to set value in Redis");

    HttpResponse::Created().json(response)
}

#[get("/authorize")]
async fn authorize_handler() -> impl Responder {
    HttpResponse::Ok().body("auth authorize endpoint")
}

#[post("/token")]
async fn token_handler() -> impl Responder {
    HttpResponse::Ok().body("auth token endpoint")
}