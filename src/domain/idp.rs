use uuid::Uuid;

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

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct IdpUser {
    pub id: Uuid,
    pub name: Option<String>,
    #[serde(rename = "familyName")]
    pub family_name: Option<String>,
    pub email: Option<String>,
    pub birthdate: Option<String>,
    pub gender: Option<String>,
    pub status: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}
