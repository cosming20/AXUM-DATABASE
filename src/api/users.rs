use crate::app::models::{User, RegisterRequest};
use leptos::prelude::*;

#[server(GetUsers, "/api")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::users::SqlUser;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        let users = SqlUser::get_all_users(&mut conn)
            .map_err(|e| ServerFnError::new(format!("Failed to get users: {}", e)))?;
        
        Ok(SqlUser::to_app_models(users))
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(CreateUser, "/api")]
pub async fn create_user(request: RegisterRequest) -> Result<User, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::users::SqlUser;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        // Check if user already exists
        if let Ok(_) = SqlUser::get_user_by_email(&mut conn, &request.email) {
            return Err(ServerFnError::new("User with this email already exists"));
        }
        
        let user = SqlUser::create_user(&mut conn, request)
            .map_err(|e| ServerFnError::new(format!("Failed to create user: {}", e)))?;
        
        Ok(user.to_app_model())
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(GetUserById, "/api")]
pub async fn get_user_by_id(user_id: i32) -> Result<Option<User>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::users::SqlUser;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        match SqlUser::get_user_by_id(&mut conn, user_id) {
            Ok(user) => Ok(Some(user.to_app_model())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(ServerFnError::new(format!("Failed to get user: {}", e))),
        }
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(UpdateUser, "/api")]
pub async fn update_user(
    user_id: i32,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::users::SqlUser;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        SqlUser::update_user(&mut conn, user_id, first_name, last_name, phone)
            .map_err(|e| ServerFnError::new(format!("Failed to update user: {}", e)))?;
        
        Ok(())
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(DeactivateUser, "/api")]
pub async fn deactivate_user(user_id: i32) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::users::SqlUser;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        SqlUser::deactivate_user(&mut conn, user_id)
            .map_err(|e| ServerFnError::new(format!("Failed to deactivate user: {}", e)))?;
        
        Ok(())
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
} 