use std::sync::Arc;
use actix_web::HttpResponse;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::controller::ControllerInterface;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::use_cases::auth::authorize::AuthorizeUseCase;
use crate::application::use_cases::auth::authorize_continue::AuthorizeContinueUseCase;
use crate::dto::auth::authorize::request::AuthorizeRequest;
use crate::utils::api_error::ApiError;

pub struct AuthorizeController {
    pub cache: Arc<RedisCache>,
    pub repository: Arc<OAuthSessionRepository>,
}

impl ControllerInterface for AuthorizeController {
    type Data = AuthorizeRequest;
    type Result = HttpResponse;

    async fn handle(&self, data: Self::Data) -> Self::Result {
        if !data.auth_token.is_none()  {
            let case = AuthorizeContinueUseCase {
                cache: self.cache.clone(),
                repository: self.repository.clone(),
            };

            return self.format_result(case.handle(data).await)
        }

        let case = AuthorizeUseCase {
            cache: self.cache.clone(),
            repository: self.repository.clone()
        };

        self.format_result(case.handle(data).await)
    }
}

impl AuthorizeController {
    fn format_result(&self, result: (Result<String, String>, u16)) -> HttpResponse {
        match result.1 {
            303 => HttpResponse::SeeOther().append_header(("Location", result.0.unwrap())).finish(),
            _ => HttpResponse::build(
                actix_web::http::StatusCode::from_u16(result.1).unwrap()
            ).json(ApiError::new(result.0.err().unwrap()))
        }
    }
}