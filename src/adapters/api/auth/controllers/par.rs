use std::sync::Arc;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::par::ParUseCase;
use crate::dto::auth::par::request::ParRequest;
use crate::dto::auth::par::response::ParResponse;

pub struct ParController {
    pub cache: Arc<RedisCache>,
    pub repository: Arc<OAuthClientRepository>
}

impl ControllerInterface for ParController {
    type Data = ParRequest;
    type Result = Result<ParResponse, String>;
    async fn handle(&self, data: Self::Data) -> Self::Result {
        let case = ParUseCase {
            cache: self.cache.clone(),
            repository: self.repository.clone(),
        };

        case.handle(data).await
    }
}

