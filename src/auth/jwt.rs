#[cfg(feature = "ssr")]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use chrono::{Duration, Utc};
#[cfg(feature = "ssr")]
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub email: String,    // User email
    pub role: String,     // User role (customer, staff, admin)
    pub exp: usize,       // Expiration time
    pub iat: usize,       // Issued at
}

#[derive(Debug)]
pub enum JwtError {
    InvalidToken,
    ExpiredToken,
    MissingSecret,
    EncodingError,
}

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtError::InvalidToken => write!(f, "Invalid token"),
            JwtError::ExpiredToken => write!(f, "Token has expired"),
            JwtError::MissingSecret => write!(f, "JWT secret not found"),
            JwtError::EncodingError => write!(f, "Error encoding token"),
        }
    }
}

impl std::error::Error for JwtError {}

/// Create a JWT token for a user (server-side only)
#[cfg(feature = "ssr")]
pub fn create_jwt_token(
    user_id: i32,
    email: String,
    role: String,
    expires_in_hours: i64,
) -> Result<String, JwtError> {
    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|_| JwtError::MissingSecret)?;

    let now = Utc::now();
    let exp = (now + Duration::hours(expires_in_hours)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email,
        role,
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| JwtError::EncodingError)
}

/// Validate and decode a JWT token (server-side only)
#[cfg(feature = "ssr")]
pub fn validate_jwt_token(token: &str) -> Result<Claims, JwtError> {
    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|_| JwtError::MissingSecret)?;

    let validation = Validation::new(Algorithm::HS256);
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|err| {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::ExpiredToken,
            _ => JwtError::InvalidToken,
        }
    })
}

/// Create a short-lived token (for login sessions) - server-side only
#[cfg(feature = "ssr")]
pub fn create_session_token(user_id: i32, email: String, role: String) -> Result<String, JwtError> {
    create_jwt_token(user_id, email, role, 24) // 24 hours
}

/// Create a long-lived token (for "remember me" functionality) - server-side only
#[cfg(feature = "ssr")]
pub fn create_refresh_token(user_id: i32, email: String, role: String) -> Result<String, JwtError> {
    create_jwt_token(user_id, email, role, 24 * 7) // 7 days
}

// Client-side stubs (for WebAssembly compatibility)
#[cfg(not(feature = "ssr"))]
pub fn create_jwt_token(
    _user_id: i32,
    _email: String,
    _role: String,
    _expires_in_hours: i64,
) -> Result<String, JwtError> {
    Err(JwtError::EncodingError)
}

#[cfg(not(feature = "ssr"))]
pub fn validate_jwt_token(_token: &str) -> Result<Claims, JwtError> {
    Err(JwtError::InvalidToken)
}

#[cfg(not(feature = "ssr"))]
pub fn create_session_token(_user_id: i32, _email: String, _role: String) -> Result<String, JwtError> {
    Err(JwtError::EncodingError)
}

#[cfg(not(feature = "ssr"))]
pub fn create_refresh_token(_user_id: i32, _email: String, _role: String) -> Result<String, JwtError> {
    Err(JwtError::EncodingError)
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_jwt_creation_and_validation() {
        // Set test JWT secret
        env::set_var("JWT_SECRET", "test_secret_key_for_testing");

        let user_id = 123;
        let email = "test@example.com".to_string();
        let role = "customer".to_string();

        // Create token
        let token = create_session_token(user_id, email.clone(), role.clone()).unwrap();
        assert!(!token.is_empty());

        // Validate token
        let claims = validate_jwt_token(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
    }

    #[test]
    fn test_invalid_token() {
        env::set_var("JWT_SECRET", "test_secret_key_for_testing");
        
        let result = validate_jwt_token("invalid.token.here");
        assert!(result.is_err());
    }
} 