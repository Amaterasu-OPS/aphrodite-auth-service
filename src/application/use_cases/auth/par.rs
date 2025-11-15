use std::sync::Arc;
use deadpool_redis::redis::{AsyncCommands};
use crate::adapters::spi::cache::redis::RedisCache;
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::domain::oauth_client::OauthClient;
use crate::dto::auth::par::request::ParRequest;
use crate::dto::auth::par::response::ParResponse;
use crate::utils::entropy::entropy_total_bits;

pub struct ParUseCase {
    pub cache: Arc<RedisCache>,
    pub repository: Arc<OAuthClientRepository>
}

impl UseCaseInterface for ParUseCase {
    type T = ParRequest;
    type U = ParResponse;

    async fn handle(&self, data: ParRequest) -> Result<ParResponse, String> {
        let arc_data = Arc::new(data);

        match self.validate_state(arc_data.clone()) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        let client = match self.get_client(Arc::clone(&arc_data)).await {
            Ok(e) => e,
            Err(err) => {
                return Err(format!("Error getting client: {}", err));
            }
        };

        match self.validate_uris(Arc::clone(&arc_data), &client) {
            Ok(_) => {}
            Err(err) => {
                return Err(format!("Error validating redirect URIs: {}", err));
            }
        };

        match self.validate_scopes(Arc::clone(&arc_data), &client) {
            Ok(_) => {}
            Err(err) => {
                return Err(format!("Error validating scopes: {}", err));
            }
        }

        let exp = 30;
        let request_uri = String::from("urn:ietf:params:oauth:request_uri:") + &uuid::Uuid::new_v4().to_string();
        let response = ParResponse {
            request_uri: request_uri.clone(),
            expires_in: exp,
        };

        let mut conn = match self.cache.pool.get()
            .await {
            Ok(conn) => conn,
            Err(_) => {
                return Err(String::from("Failed to get connection from pool"));
            }
        };

        let value = serde_json::to_string(&arc_data).unwrap();

        match conn.set_ex::<String, String, ()>(request_uri, value, exp)
            .await {
            Ok(_) => {}
            Err(_) => {
                return Err(String::from("Failed to set value"));
            }
        };

        Ok(response)
    }
}

impl ParUseCase {
    async fn get_client(&self, data: Arc<ParRequest>) -> Result<OauthClient, String> {
        self.repository.get_by_slug_secret(data.client_id.clone(), data.client_secret.clone()).await
    }

    fn validate_uris(&self, data: Arc<ParRequest>, client: &OauthClient) -> Result<(), String> {
        if data.redirect_uri.is_empty() {
            return Err(String::from("Invalid redirect URI"));
        }

        if client.urls.is_none() {
            return Err(String::from("Invalid redirect URI"));
        }

        let urls = client.urls.clone().unwrap();

        if !urls.contains(&data.redirect_uri) {
            return Err(String::from("Invalid redirect URI"));
        }

        Ok(())
    }

    fn validate_scopes(&self, data: Arc<ParRequest>, client: &OauthClient) -> Result<(), String> {
        if data.scope.is_empty() {
            return Err(String::from("Invalid scopes"));
        }

        if client.scopes.is_none() {
            return Err(String::from("Invalid scopes"));
        }

        let scopes = client.scopes.clone().unwrap();

        let requested_scopes = data.scope.split(" ").collect::<Vec<&str>>();

        for scope in requested_scopes {
            if !scopes.contains(&scope.to_owned()) {
                return Err(String::from("Invalid scopes"));
            }
        }

        Ok(())
    }

    fn validate_state(&self, data: Arc<ParRequest>) -> Result<(), String> {
        if data.response_type != "code" {
            return Err(String::from("Invalid response type"));
        }

        if data.code_challenge_method != "S256" {
            return Err(String::from("Invalid code challenge method"));
        }

        if data.state.is_empty() {
            return Err(String::from("Invalid state"));
        }

        if data.code_challenge.is_empty() {
            return Err(String::from("Invalid code challenge"));
        }

        // check data.state entropy
        if entropy_total_bits(data.state.clone().as_str()) < 64.0 {
            return Err(String::from("Invalid state entropy"));
        }

        Ok(())
    }
}