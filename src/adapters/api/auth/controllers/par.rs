use std::sync::Arc;
use actix_web::HttpResponse;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::par::ParUseCase;
use crate::dto::auth::par::request::ParRequest;
use crate::utils::api_error::ApiError;

pub struct ParController {
    pub cache: Arc<RedisCache>,
    pub repository: Arc<OAuthClientRepository>
}

impl ControllerInterface for ParController {
    type Data = ParRequest;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        let case = ParUseCase {
            cache: self.cache.clone(),
            repository: self.repository.clone(),
        };

        let result = case.handle(data).await;

        match result.1 {
            201 => HttpResponse::Created().json(result.0.unwrap()),
            _ => HttpResponse::build(
                actix_web::http::StatusCode::from_u16(result.1).unwrap()
            ).json(ApiError::new(result.0.err().unwrap()))
        }
    }
}

