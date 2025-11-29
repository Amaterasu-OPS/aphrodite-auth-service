use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::DateTime;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::{rng, RngCore};
use crate::dto::auth::token::access_token::AccessToken;
use crate::utils::hasher::hash_sha512;

pub fn generate_access_token(
    scopes: Vec<String>,
    now: DateTime<chrono::Utc>,
    jwt_iss: String,
    session_id: String,
    user_id: uuid::Uuid,
    client_id: String,
    encoding_key: EncodingKey
) -> Result<String, String> {
    let id = hash_sha512(uuid::Uuid::new_v4().to_string().as_str());
    let exp = now.timestamp() + 4 * 60 * 60;

    let token = AccessToken {
        scopes,
        sub: user_id.clone(),
        exp: exp.clone() as usize,
        iat: now.timestamp() as usize,
        iss: jwt_iss,
        aud: "".to_string(),
        jti: id.clone(),
        sid: session_id,
        client_id,
        auth_time: now.timestamp() as usize,
    };

    let Ok(result) = encode(
        &Header::new(Algorithm::RS256),
        &token,
        &encoding_key
    ) else {
        return Err(String::from("Failed to encode JWT"));
    };

    Ok(result)
}

pub fn generate_refresh_token() -> String {
    let mut buf = [0u8; 64];
    rng().fill_bytes(&mut buf);
    URL_SAFE_NO_PAD.encode(&buf)
}