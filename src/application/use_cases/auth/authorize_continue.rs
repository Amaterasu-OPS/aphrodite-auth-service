use std::sync::Arc;
use redis::AsyncCommands;
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::spi::repository::RepositoryInterface;
use crate::dto::auth::authorize::request::AuthorizeRequest;

pub struct AuthorizeContinueUseCase {
    pub cache: Arc<RedisCache>,
    pub repository: Arc<OAuthSessionRepository>,
}

impl UseCaseInterface for AuthorizeContinueUseCase {
    type T = AuthorizeRequest;
    type U = String;

    async fn handle(&self, data: Self::T) -> (Result<Self::U, String>, u16) {
        let arc_data = Arc::new(data);
    
        match self.validate_query(arc_data.clone()) {
            Ok(_) => {},
            Err(e) => return (Err(e), 400)
        };

        let (session_uuid, user_uuid) = match self.get_session_and_user_uuid(arc_data.clone()) {
            Ok(uuids) => uuids,
            Err(e) => return (Err(e.0), e.1)
        };

        let mut session = match self.repository.get(session_uuid).await {
            Ok(result) => result,
            Err(e) => return (Err(e), 500)
        };

        if session.user_id.is_some() {
            return (Err("Session already authorized".to_string()), 422)
        }

        let token = uuid::Uuid::new_v4();

        session.user_id = Some(user_uuid);

        match self.repository.edit(session.id.unwrap(), session.clone(), vec![
            "user_id",
        ]).await {
            Ok(_) => {},
            Err(e) => return (Err(e), 500)
        };

        let mut conn = match self.cache.get_pool().await {
            Ok(conn) => conn,
            Err(e) => return (Err(e), 500)
        };

        match conn.set_ex::<String, String, ()>(token.to_string(), user_uuid.to_string(), 60 * 5)
            .await {
            Ok(_) => {}
            Err(_) => return (Err(String::from("Failed to set value")), 500)
        };

        (Ok(session.redirect_uri.unwrap() + "?code=" + &token.to_string() + "&state=" + &session.state.unwrap()), 303)
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
    
    fn get_session_and_user_uuid(&self, data: Arc<AuthorizeRequest>) -> Result<(uuid::Uuid, uuid::Uuid), (String, u16)>  {
        let session_id = data.session_id.as_ref().unwrap();
        let user_id = data.user_id.as_ref().unwrap();

        let session_uuid = match uuid::Uuid::parse_str(session_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(("Invalid session ID".to_string(), 422))
        };

        let user_uuid = match uuid::Uuid::parse_str(user_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(("Invalid user ID".to_string(), 422))
        };
        
        Ok((session_uuid, user_uuid))
    }
}