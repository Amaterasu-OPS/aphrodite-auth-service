use std::env;
use std::sync::Arc;
use actix_web::http::StatusCode;
use jsonwebtoken::EncodingKey;
use crate::adapters::spi::gateways::idp::IdpGateway;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::adapters::spi::repositories::oauth_session::OAuthSessionRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::application::spi::repository::RepositoryInterface;
use crate::dto::auth::token::request::TokenRefreshRequest;
use crate::dto::auth::token::response::TokenResponse;
use crate::utils::api_response::{ApiError, ApiSuccess};
use crate::utils::hasher::hash_sha256;
use crate::adapters::spi::repositories::oauth_token::OAuthTokenRepository;
use crate::domain::idp_id_token::IdPIdTokenPayload;
use crate::domain::oauth_client::OauthClient;
use crate::domain::oauth_session::OauthSession;
use crate::domain::oauth_token::OauthToken;
use crate::utils::token::{generate_access_token, generate_refresh_token};

pub struct TokenRefreshUseCase {
    repository: Arc<OAuthSessionRepository>,
    token_repository: Arc<OAuthTokenRepository>,
    client_repository: Arc<OAuthClientRepository>,
    idp_gateway: Arc<IdpGateway>,
}

impl UseCaseInterface for TokenRefreshUseCase {
    type Request = TokenRefreshRequest;
    type Response = TokenResponse;

    async fn handle(&self, data: Self::Request) -> Result<ApiSuccess<Self::Response>, ApiError> {
        let arc_data = Arc::new(data);

        if let Err(e) = self.validate_request(arc_data.clone()) {
            return Err(e);
        }

        let (jwt_iss, encoding_key) = match self.validate_envs() {
            Ok(e) => e,
            Err(e) => return Err(e)
        };

        let Ok(token) = self.token_repository.get_by_refresh_token(hash_sha256(arc_data.refresh_token.clone().as_str())).await else {
            return Err(ApiError::new(String::from("Invalid refresh token"), StatusCode::BAD_REQUEST));
        };

        let Ok(repo_session) = self.repository.get(token.session_id.unwrap()).await else {
            return Err(ApiError::new(String::from("Session not found"), StatusCode::BAD_REQUEST));
        };
        
        let scopes = repo_session.scopes.clone().unwrap_or(vec![]);

        if !scopes.contains(&"offline_access".to_string()) {
            return Err(ApiError::new(String::from("offline_access is required for refresh token"), StatusCode::BAD_REQUEST));
        }

        let Ok(repo_client) = self.client_repository.get_by_slug(repo_session.client_id.clone().unwrap()).await else {
            return Err(ApiError::new(String::from("Client not found"), StatusCode::BAD_REQUEST));
        };

        if let Err(e) = self.validate_client(arc_data.clone(), repo_session.clone(), repo_client.clone()) {
            return Err(e);
        }

        let id_token = match self.idp_gateway.get_id_token_v1(IdPIdTokenPayload {
            user_id: repo_session.user_id.clone().unwrap().to_string(),
            client_id: repo_session.client_id.clone().unwrap().to_string(),
            scopes: repo_session.scopes.clone().unwrap_or(vec![])
        }).await {
            Ok(e) => e,
            Err(e) => return Err(e)
        };

        let Ok(access_token) = generate_access_token(
            scopes,
            chrono::Utc::now(),
            jwt_iss,
            repo_session.id.unwrap().to_string(),
            repo_session.user_id.unwrap().clone(),
            repo_session.client_id.unwrap().to_string(),
            encoding_key
        ) else {
            return Err(ApiError::new(String::from("Failed to generate access token"), StatusCode::INTERNAL_SERVER_ERROR));
        };

        let refresh_token = generate_refresh_token();

        if let Err(_) = self.token_repository.edit(token.id.unwrap(), OauthToken {
            id: None,
            session_id: None,
            access_token: Some(hash_sha256(access_token.clone().as_str())),
            refresh_token: Some(hash_sha256(refresh_token.clone().as_str())),
            status: None,
            created_at: None,
            updated_at: None,
        }, vec!["access_token", "refresh_token"]).await {
            return Err(ApiError::new(String::from("Failed to save token"), StatusCode::INTERNAL_SERVER_ERROR))
        }

        Ok(ApiSuccess::new(
            TokenResponse {
                access_token,
                refresh_token,
                id_token,
            },
            StatusCode::OK
        ))
    }
}

impl TokenRefreshUseCase {
    pub fn new(
        repository: Arc<OAuthSessionRepository>,
        token_repository: Arc<OAuthTokenRepository>,
        client_repository: Arc<OAuthClientRepository>,
        idp_gateway: Arc<IdpGateway>,
    ) -> Self {
        Self { repository, token_repository, client_repository, idp_gateway  }
    }

    fn validate_request(&self, data: Arc<TokenRefreshRequest>) -> Result<(), ApiError> {
        if data.grant_type != "refresh_token" {
            return Err(ApiError::new(String::from("Invalid grant type"), StatusCode::BAD_REQUEST));
        }

        Ok(())
    }

    fn validate_client(&self, data: Arc<TokenRefreshRequest>, session: OauthSession, client: OauthClient) -> Result<(), ApiError> {
        if session.client_id.unwrap() != data.client_id {
            return Err(ApiError::new(String::from("Invalid client"), StatusCode::BAD_REQUEST));
        };

        if data.client_secret != client.secret.unwrap() {
            return Err(ApiError::new(String::from("Invalid client"), StatusCode::BAD_REQUEST));
        }

        Ok(())
    }

    fn validate_envs(&self) -> Result<(String, EncodingKey), ApiError> {
        let Ok(jwt_iss) = env::var("JWT_ISSUER") else {
            return Err(ApiError::new(String::from("JWT_ISSUER not found"), StatusCode::INTERNAL_SERVER_ERROR));
        };

        let Ok(jwt_pk) = env::var("JWT_PRIVATE_KEY") else {
            return Err(ApiError::new(String::from("JWT_PRIVATE_KEY not found"), StatusCode::INTERNAL_SERVER_ERROR));
        };

        let Ok(encoding_key) = EncodingKey::from_rsa_pem(jwt_pk.replace("\\n", "\n").as_bytes()) else {
            return Err(ApiError::new(String::from("Failed to parse JWT_PRIVATE_KEY"), StatusCode::INTERNAL_SERVER_ERROR));
        };

        Ok((jwt_iss, encoding_key))
    }
}