use crate::app::models::{Transaction, TransferRequest};
use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

#[server(GetTransactions, "/api")]
pub async fn get_transactions() -> Result<Vec<Transaction>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::transactions::SqlTransaction;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        let transactions = SqlTransaction::find_all(&mut conn)
            .map_err(|e| ServerFnError::new(format!("Failed to get transactions: {}", e)))?;
        
        let app_transactions: Vec<Transaction> = transactions.into_iter().map(|txn| {
            Transaction {
                id: txn.id,
                uuid: txn.uuid,
                from_account_id: txn.from_account_id,
                to_account_id: txn.to_account_id,
                amount: Decimal::from_str(&txn.amount.to_string()).unwrap_or_default(),
                currency: txn.currency,
                transaction_type: crate::app::models::TransactionType::from(txn.transaction_type),
                description: txn.description,
                status: crate::app::models::TransactionStatus::from(txn.status),
                reference_number: txn.reference_number,
                from_account_number: None, // TODO: Join with accounts table to get account numbers
                to_account_number: None, // TODO: Join with accounts table to get account numbers
                created_at: chrono::DateTime::from_naive_utc_and_offset(txn.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(txn.updated_at, chrono::Utc),
            }
        }).collect();
        
        Ok(app_transactions)
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(GetTransactionsByAccount, "/api")]
pub async fn get_transactions_by_account(account_id: i32) -> Result<Vec<Transaction>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::transactions::SqlTransaction;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        let transactions = SqlTransaction::find_by_account(&mut conn, account_id)
            .map_err(|e| ServerFnError::new(format!("Failed to get account transactions: {}", e)))?;
        
        let app_transactions: Vec<Transaction> = transactions.into_iter().map(|txn| {
            Transaction {
                id: txn.id,
                uuid: txn.uuid,
                from_account_id: txn.from_account_id,
                to_account_id: txn.to_account_id,
                amount: Decimal::from_str(&txn.amount.to_string()).unwrap_or_default(),
                currency: txn.currency,
                transaction_type: crate::app::models::TransactionType::from(txn.transaction_type),
                description: txn.description,
                status: crate::app::models::TransactionStatus::from(txn.status),
                reference_number: txn.reference_number,
                from_account_number: None, // TODO: Join with accounts table to get account numbers
                to_account_number: None, // TODO: Join with accounts table to get account numbers
                created_at: chrono::DateTime::from_naive_utc_and_offset(txn.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(txn.updated_at, chrono::Utc),
            }
        }).collect();
        
        Ok(app_transactions)
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(CreateTransfer, "/api")]
pub async fn create_transfer(request: TransferRequest) -> Result<Transaction, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::transactions::SqlTransaction;
        use crate::db::models::accounts::SqlAccount;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        // Start a transaction for atomic operations
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // Get source account
            let from_account = SqlAccount::find_by_id(conn, request.from_account_id)?;
            let current_balance: f64 = from_account.balance.to_string().parse().unwrap_or(0.0);
            let request_amount: f64 = request.amount.to_string().parse().unwrap_or(0.0);
            
            // Check if sufficient funds
            if current_balance < request_amount {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            
            // Find destination account by account number
            let to_account = SqlAccount::find_by_account_number(conn, &request.to_account_number)?;
            
            // Update source account balance
            SqlAccount::update_balance(conn, request.from_account_id, current_balance - request_amount)?;
            
            // Update destination account balance
            let to_balance: f64 = to_account.balance.to_string().parse().unwrap_or(0.0);
            SqlAccount::update_balance(conn, to_account.id, to_balance + request_amount)?;
            
            // Create transaction record
            let transaction = SqlTransaction::create_transfer(
                conn,
                request.from_account_id,
                to_account.id,
                request_amount,
                request.description,
            )?;
            
            Ok(transaction)
        })
        .map_err(|e| {
            match e {
                diesel::result::Error::RollbackTransaction => {
                    ServerFnError::new("Insufficient funds for transfer")
                }
                diesel::result::Error::NotFound => {
                    ServerFnError::new("Destination account not found")
                }
                _ => ServerFnError::new(format!("Transfer failed: {}", e))
            }
        })
        .map(|txn| Transaction {
            id: txn.id,
            uuid: txn.uuid,
            from_account_id: txn.from_account_id,
            to_account_id: txn.to_account_id,
            amount: Decimal::from_str(&txn.amount.to_string()).unwrap_or_default(),
            currency: txn.currency,
            transaction_type: crate::app::models::TransactionType::from(txn.transaction_type),
            description: txn.description,
            status: crate::app::models::TransactionStatus::from(txn.status),
            reference_number: txn.reference_number,
            from_account_number: None, // TODO: Join with accounts table to get account numbers
            to_account_number: None, // TODO: Join with accounts table to get account numbers
            created_at: chrono::DateTime::from_naive_utc_and_offset(txn.created_at, chrono::Utc),
            updated_at: chrono::DateTime::from_naive_utc_and_offset(txn.updated_at, chrono::Utc),
        })
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(CreateDeposit, "/api")]
pub async fn create_deposit(account_id: i32, amount: f64, description: Option<String>) -> Result<Transaction, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::transactions::SqlTransaction;
        use crate::db::models::accounts::SqlAccount;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // Get current balance
            let account = SqlAccount::find_by_id(conn, account_id)?;
            let current_balance: f64 = account.balance.to_string().parse().unwrap_or(0.0);
            
            // Update account balance
            SqlAccount::update_balance(conn, account_id, current_balance + amount)?;
            
            // Create transaction record
            let transaction = SqlTransaction::create_deposit(conn, account_id, amount, description)?;
            
            Ok(transaction)
        })
        .map_err(|e| ServerFnError::new(format!("Deposit failed: {}", e)))
        .map(|txn| Transaction {
            id: txn.id,
            uuid: txn.uuid,
            from_account_id: txn.from_account_id,
            to_account_id: txn.to_account_id,
            amount: Decimal::from_str(&txn.amount.to_string()).unwrap_or_default(),
            currency: txn.currency,
            transaction_type: crate::app::models::TransactionType::from(txn.transaction_type),
            description: txn.description,
            status: crate::app::models::TransactionStatus::from(txn.status),
            reference_number: txn.reference_number,
            from_account_number: None, // TODO: Join with accounts table to get account numbers
            to_account_number: None, // TODO: Join with accounts table to get account numbers
            created_at: chrono::DateTime::from_naive_utc_and_offset(txn.created_at, chrono::Utc),
            updated_at: chrono::DateTime::from_naive_utc_and_offset(txn.updated_at, chrono::Utc),
        })
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(CreateWithdrawal, "/api")]
pub async fn create_withdrawal(account_id: i32, amount: f64, description: Option<String>) -> Result<Transaction, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::transactions::SqlTransaction;
        use crate::db::models::accounts::SqlAccount;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // Get current balance
            let account = SqlAccount::find_by_id(conn, account_id)?;
            let current_balance: f64 = account.balance.to_string().parse().unwrap_or(0.0);
            
            // Check if sufficient funds
            if current_balance < amount {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            
            // Update account balance
            SqlAccount::update_balance(conn, account_id, current_balance - amount)?;
            
            // Create transaction record
            let transaction = SqlTransaction::create_withdrawal(conn, account_id, amount, description)?;
            
            Ok(transaction)
        })
        .map_err(|e| {
            match e {
                diesel::result::Error::RollbackTransaction => {
                    ServerFnError::new("Insufficient funds for withdrawal")
                }
                _ => ServerFnError::new(format!("Withdrawal failed: {}", e))
            }
        })
        .map(|txn| Transaction {
            id: txn.id,
            uuid: txn.uuid,
            from_account_id: txn.from_account_id,
            to_account_id: txn.to_account_id,
            amount: Decimal::from_str(&txn.amount.to_string()).unwrap_or_default(),
            currency: txn.currency,
            transaction_type: crate::app::models::TransactionType::from(txn.transaction_type),
            description: txn.description,
            status: crate::app::models::TransactionStatus::from(txn.status),
            reference_number: txn.reference_number,
            from_account_number: None, // TODO: Join with accounts table to get account numbers
            to_account_number: None, // TODO: Join with accounts table to get account numbers
            created_at: chrono::DateTime::from_naive_utc_and_offset(txn.created_at, chrono::Utc),
            updated_at: chrono::DateTime::from_naive_utc_and_offset(txn.updated_at, chrono::Utc),
        })
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
} 