use std::sync::Arc;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::par::ParUseCase;
use crate::dto::auth::par::request::ParRequest;
use crate::utils::api_response::ApiErrorResponse;

pub struct ParController {
    cache: Arc<RedisCache>,
    repository: Arc<OAuthClientRepository>
}

impl ControllerInterface for ParController {
    type Data = ParRequest;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        match ParUseCase::new(
            self.cache.clone(),
            self.repository.clone(),
        ).handle(data).await {
            Ok(e) => HttpResponse::Created().json(e.data),
            Err(e) => HttpResponse::build(StatusCode::from_u16(e.status_code).unwrap()).json(ApiErrorResponse::new(e.error)),
        }
    }
}

impl ParController {
    pub fn new(cache: Arc<RedisCache>, repository: Arc<OAuthClientRepository>) -> Self {
        Self { cache, repository  }
    }
}
