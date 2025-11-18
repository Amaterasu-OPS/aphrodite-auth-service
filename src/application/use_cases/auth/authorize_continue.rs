use std::sync::Arc;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::spi::repository::RepositoryInterface;
use crate::dto::auth::authorize::request::AuthorizeRequest;

pub struct AuthorizeContinueUseCase {
    pub repository: Arc<OAuthSessionRepository>,
}

impl UseCaseInterface for AuthorizeContinueUseCase {
    type T = AuthorizeRequest;
    type U = String;

    async fn handle(&self, data: Self::T) -> Result<Self::U, String> {
        let arc_data = Arc::new(data);
        match self.validate_query(arc_data.clone()) {
            Ok(_) => {},
            Err(e) => {
                return Err(e)
            }
        };

        let session_id = arc_data.session_id.as_ref().unwrap();
        let user_id = arc_data.user_id.as_ref().unwrap();

        let mut session = match self.repository.get(uuid::Uuid::parse_str(session_id).map_err(|_| "Invalid session ID".to_string())?).await {
            Ok(result) => result,
            Err(e) => return Err(e)
        };

        if session.authorization_code.is_some() {
            return Err("Session already authorized".to_string())
        }

        let token = uuid::Uuid::new_v4();

        session.user_id = Some(uuid::Uuid::parse_str(user_id).map_err(|_| "Invalid user ID".to_string())?);
        session.authorization_code = Some(token.to_string());

        match self.repository.edit(session.id.unwrap(), session.clone(), vec![
            "user_id",
            "authorization_code"
        ]).await {
            Ok(_) => {},
            Err(e) => return Err(e)
        };

        Ok(session.redirect_uri.unwrap() + "?code=" + &token.to_string() + "&state=" + &session.state.unwrap())
    }
}

#[allow(unused)]
impl AuthorizeContinueUseCase {
    fn validate_query(&self, data: Arc<AuthorizeRequest>) -> Result<(), String> {
        if data.auth_token.is_none() {
            return Err("Missing auth token".to_string())
        }

        if data.session_id.is_none() {
            return Err("Missing session id".to_string())
        }

        if data.user_id.is_none() {
            return Err("Missing user id".to_string())
        }

        Ok(())
    }
}