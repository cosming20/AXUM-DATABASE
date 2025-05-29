use crate::app::models::{Account, CreateAccountRequest};
use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

#[server(GetAccounts, "/api")]
pub async fn get_accounts() -> Result<Vec<Account>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::accounts::SqlAccount;
        use crate::establish_connection;
        
        let mut conn = establish_connection();
        
        let accounts = SqlAccount::find_all(&mut conn)
            .map_err(|e| ServerFnError::new(format!("Failed to get accounts: {}", e)))?;
        
        // Convert to app models
        let app_accounts: Vec<Account> = accounts.into_iter().map(|acc| {
            Account {
                id: acc.id,
                uuid: acc.uuid,
                user_id: acc.user_id,
                branch_id: acc.branch_id,
                account_number: acc.account_number,
                account_type: crate::app::models::AccountType::from(acc.account_type),
                balance: Decimal::from_str(&acc.balance.to_string()).unwrap_or_default(),
                currency: acc.currency,
                is_active: acc.is_active,
                user_name: None, // TODO: Join with users table to get name
                branch_name: None, // TODO: Join with branches table to get name
                created_at: chrono::DateTime::from_naive_utc_and_offset(acc.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(acc.updated_at, chrono::Utc),
            }
        }).collect();
        
        Ok(app_accounts)
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(GetAccountsByUser, "/api")]
pub async fn get_accounts_by_user(user_id: i32) -> Result<Vec<Account>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::accounts::SqlAccount;
        use crate::establish_connection;
        
        let mut conn = establish_connection();
        
        let accounts = SqlAccount::find_by_user(&mut conn, user_id)
            .map_err(|e| ServerFnError::new(format!("Failed to get user accounts: {}", e)))?;
        
        let app_accounts: Vec<Account> = accounts.into_iter().map(|acc| {
            Account {
                id: acc.id,
                uuid: acc.uuid,
                user_id: acc.user_id,
                branch_id: acc.branch_id,
                account_number: acc.account_number,
                account_type: crate::app::models::AccountType::from(acc.account_type),
                balance: Decimal::from_str(&acc.balance.to_string()).unwrap_or_default(),
                currency: acc.currency,
                is_active: acc.is_active,
                user_name: None, // TODO: Join with users table to get name
                branch_name: None, // TODO: Join with branches table to get name
                created_at: chrono::DateTime::from_naive_utc_and_offset(acc.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(acc.updated_at, chrono::Utc),
            }
        }).collect();
        
        Ok(app_accounts)
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(CreateAccount, "/api")]
pub async fn create_account(request: CreateAccountRequest) -> Result<Account, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::accounts::SqlAccount;
        use crate::establish_connection;
        
        let mut conn = establish_connection();
        
        // Convert Decimal to f64 for database operation
        let initial_balance = request.initial_balance.to_string().parse::<f64>().unwrap_or(0.0);
        
        // Use branch_id = 1 as default (Main Branch)
        let account = SqlAccount::create_account(
            &mut conn,
            request.user_id,
            1, // Default to main branch
            request.account_type.to_string(),
            Some(initial_balance),
        ).map_err(|e| ServerFnError::new(format!("Failed to create account: {}", e)))?;
        
        Ok(Account {
            id: account.id,
            uuid: account.uuid,
            user_id: account.user_id,
            branch_id: account.branch_id,
            account_number: account.account_number,
            account_type: crate::app::models::AccountType::from(account.account_type),
            balance: Decimal::from_str(&account.balance.to_string()).unwrap_or_default(),
            currency: account.currency,
            is_active: account.is_active,
            user_name: None, // TODO: Join with users table to get name
            branch_name: None, // TODO: Join with branches table to get name
            created_at: chrono::DateTime::from_naive_utc_and_offset(account.created_at, chrono::Utc),
            updated_at: chrono::DateTime::from_naive_utc_and_offset(account.updated_at, chrono::Utc),
        })
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(GetAccountById, "/api")]
pub async fn get_account_by_id(account_id: i32) -> Result<Option<Account>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::accounts::SqlAccount;
        use crate::establish_connection;
        
        let mut conn = establish_connection();
        
        match SqlAccount::find_by_id(&mut conn, account_id) {
            Ok(account) => Ok(Some(Account {
                id: account.id,
                uuid: account.uuid,
                user_id: account.user_id,
                branch_id: account.branch_id,
                account_number: account.account_number,
                account_type: crate::app::models::AccountType::from(account.account_type),
                balance: Decimal::from_str(&account.balance.to_string()).unwrap_or_default(),
                currency: account.currency,
                is_active: account.is_active,
                user_name: None, // TODO: Join with users table to get name
                branch_name: None, // TODO: Join with branches table to get name
                created_at: chrono::DateTime::from_naive_utc_and_offset(account.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(account.updated_at, chrono::Utc),
            })),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(ServerFnError::new(format!("Failed to get account: {}", e))),
        }
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(UpdateAccountBalance, "/api")]
pub async fn update_account_balance(account_id: i32, new_balance: f64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::accounts::SqlAccount;
        use crate::establish_connection;
        
        let mut conn = establish_connection();
        
        SqlAccount::update_balance(&mut conn, account_id, new_balance)
            .map_err(|e| ServerFnError::new(format!("Failed to update balance: {}", e)))?;
        
        Ok(())
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
} 