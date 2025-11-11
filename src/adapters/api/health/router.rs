use actix_web::{HttpResponse, Responder, Scope, get, web};

pub fn health_router() -> Scope {
    web::scope("/health").service(health_handler)
}

#[get("")]
async fn health_handler() -> impl Responder {
    HttpResponse::Ok().body("ok")
}