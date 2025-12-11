#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct IdPIdTokenResponse {
    #[serde(rename = "idToken")]
    pub id_token: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct IdPIdTokenRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct IdpVerifyCredentialRequest {
    pub token: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct IdpVerifyCredentialResponse {
    pub verified: bool,
}