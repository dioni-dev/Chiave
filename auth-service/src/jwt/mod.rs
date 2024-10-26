// src/jwt/mod.rs
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use crate::jwt::claims::Claims;

pub fn create_token(username: &str, secret: &str) -> String {
    let claims = Claims {
        sub: username.to_owned(),
        exp: get_expiration(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn validate_token(token: &str, secret: &str) -> bool {
    decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).is_ok()
}

fn get_expiration() -> usize {
    // Implementación para obtener el tiempo de expiración del token
}

