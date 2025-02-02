use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_access_token(
    user_id: &str,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now() + Duration::hours(1);
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn create_refresh_token(
    user_id: &str,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now() + Duration::days(7);
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn verify_token(
    token: &str,
    secret: &[u8],
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
}
