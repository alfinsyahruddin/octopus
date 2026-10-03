use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    entities::{app_config::AppConfig, app_error::AppError},
    enums::{token_type::TokenType, user_role::UserRole},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub token_type: TokenType,
    pub sid: Uuid,
    pub iat: i64,
    pub exp: i64,
}

pub fn decode_token(token: &str, config: &AppConfig) -> Result<Claims, AppError> {
    let bearer_token = token.trim().strip_prefix("Bearer ").unwrap_or(token.trim());
    decode::<Claims>(
        bearer_token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|token| token.claims)
    .map_err(|_| AppError::unauthorized("Invalid or expired token"))
}
