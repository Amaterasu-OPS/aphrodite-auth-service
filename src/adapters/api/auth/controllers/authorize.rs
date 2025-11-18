use std::sync::Arc;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::authorize::AuthorizeUseCase;
use crate::application::use_cases::auth::authorize_continue::AuthorizeContinueUseCase;
use crate::dto::auth::authorize::request::AuthorizeRequest;

pub struct AuthorizeController {
    pub cache: Arc<RedisCache>,
    pub repository: Arc<OAuthSessionRepository>,
}

impl ControllerInterface for AuthorizeController {
    type Data = AuthorizeRequest;
    type Result = Result<String, String>;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        if !data.auth_token.is_none()  {
            let case = AuthorizeContinueUseCase {
                repository: self.repository.clone(),
            };

            return case.handle(data).await;
        }

        let case = AuthorizeUseCase {
            cache: self.cache.clone(),
            repository: self.repository.clone()
        };

        case.handle(data).await
    }
}