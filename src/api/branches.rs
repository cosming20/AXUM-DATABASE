use crate::app::models::Branch;
use leptos::prelude::*;

#[server(GetBranches, "/api")]
pub async fn get_branches() -> Result<Vec<Branch>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::branches::SqlBranch;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        let branches = SqlBranch::find_all(&mut conn)
            .map_err(|e| ServerFnError::new(format!("Failed to get branches: {}", e)))?;
        
        Ok(SqlBranch::to_app_models(branches))
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(GetBranchesByBank, "/api")]
pub async fn get_branches_by_bank(bank_id: i32) -> Result<Vec<Branch>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::branches::SqlBranch;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        let branches = SqlBranch::find_by_bank(&mut conn, bank_id)
            .map_err(|e| ServerFnError::new(format!("Failed to get bank branches: {}", e)))?;
        
        Ok(SqlBranch::to_app_models(branches))
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
} 