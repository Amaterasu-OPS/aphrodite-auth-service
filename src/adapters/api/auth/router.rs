use actix_web::{HttpResponse, Responder, Scope, get, post, web};
use crate::adapters::api::auth::controllers::par::ParController;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::api::controller::ControllerInterface;
use crate::dto::auth::par::{request::ParRequest};

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
    let result = ParController {
        cache: cache.into_inner(),
        repository: repository.into_inner(),
    }
        .handle(data.into_inner())
        .await;


    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/authorize")]
async fn authorize_handler() -> impl Responder {
    HttpResponse::Ok().body("auth authorize endpoint")
}

#[post("/token")]
async fn token_handler() -> impl Responder {
    HttpResponse::Ok().body("auth token endpoint")
}