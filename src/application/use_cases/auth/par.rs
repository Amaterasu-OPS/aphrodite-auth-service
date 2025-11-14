use std::sync::Arc;
use deadpool_redis::redis::{AsyncCommands};
use crate::adapters::spi::repositories::oauth_client::OAuthClientRepository;
use crate::application::api::use_case::UseCaseInterface;
use crate::domain::oauth_client::OauthClient;
use crate::dto::auth::par::request::ParRequest;
use crate::dto::auth::par::response::ParResponse;

pub struct ParUseCase {
    pub redis_pool: Arc<deadpool_redis::Pool>,
    pub repository: Arc<OAuthClientRepository>
}

impl UseCaseInterface for ParUseCase {
    type T = ParRequest;
    type U = ParResponse;

    async fn handle(&self, data: ParRequest) -> Result<ParResponse, String> {
        let arc_data = Arc::new(data);
        let client = match self.get_client(Arc::clone(&arc_data)).await {
            Ok(e) => e,
            Err(err) => {
                return Err(format!("Error getting client: {}", err));
            }
        };

        println!("Got client: {:?}", client);

        let exp = 30;
        let request_uri = String::from("urn:ietf:params:oauth:request_uri:") + &uuid::Uuid::new_v4().to_string();
        let response = ParResponse {
            request_uri: request_uri.clone(),
            expires_in: exp,
        };

        let mut conn = match self.redis_pool.get()
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
}