#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct IdPIdToken {
    #[serde(rename = "idToken")]
    pub id_token: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct IdPIdTokenPayload {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub scopes: Vec<String>,
}