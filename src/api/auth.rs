use crate::app::models::{LoginRequest, User};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::auth::jwt::{create_session_token, validate_jwt_token};

#[server(LoginUser, "/api")]
pub async fn login_user(request: LoginRequest) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::users::SqlUser;
        use crate::establish_connection;
        
        let mut conn = establish_connection();
        
        // Find user by email
        let user = SqlUser::get_user_by_email(&mut conn, &request.email)
            .map_err(|_| ServerFnError::new("Invalid email or password"))?;
        
        // Verify password
        if !user.verify_password(&request.password) {
            return Err(ServerFnError::new("Invalid email or password"));
        }
        
        // Create JWT token
        let token = create_session_token(user.id, user.email.clone(), user.role.clone())
            .map_err(|e| ServerFnError::new(format!("Failed to create token: {}", e)))?;
        
        Ok(token)
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(LogoutUser, "/api")]
pub async fn logout_user() -> Result<(), ServerFnError> {
    // For JWT, logout is handled client-side by removing the token
    // In a more sophisticated setup, you might maintain a blacklist
    Ok(())
}

#[server(GetCurrentUser, "/api")]
pub async fn get_current_user(token: String) -> Result<Option<User>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::users::SqlUser;
        use crate::establish_connection;
        
        let mut conn = establish_connection();
        
        // Validate JWT token
        let claims = validate_jwt_token(&token)
            .map_err(|_| ServerFnError::new("Invalid token"))?;
        
        // Parse user ID from claims.sub
        let user_id: i32 = claims.sub.parse()
            .map_err(|_| ServerFnError::new("Invalid user ID in token"))?;
        
        // Get user from database
        match SqlUser::get_user_by_id(&mut conn, user_id) {
            Ok(user) => Ok(Some(user.to_app_model())),
            Err(_) => Ok(None),
        }
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(ValidateToken, "/api")]
pub async fn validate_token(token: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        match validate_jwt_token(&token) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
} 