use crate::app::models::Bank;
use leptos::prelude::*;

#[server(GetBanks, "/api")]
pub async fn get_banks() -> Result<Vec<Bank>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::banks::SqlBank;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        let banks = SqlBank::find_all(&mut conn)
            .map_err(|e| ServerFnError::new(format!("Failed to get banks: {}", e)))?;
        
        let app_banks: Vec<Bank> = banks.into_iter().map(|bank| {
            Bank {
                id: bank.id,
                name: bank.name,
                address: bank.address,
                phone: bank.phone,
                email: bank.email,
                created_at: chrono::DateTime::from_naive_utc_and_offset(bank.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(bank.updated_at, chrono::Utc),
            }
        }).collect();
        
        Ok(app_banks)
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
}

#[server(GetBankById, "/api")]
pub async fn get_bank_by_id(bank_id: i32) -> Result<Option<Bank>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::banks::SqlBank;
        use crate::establish_connection;
        use diesel::prelude::*;
        
        let mut conn = establish_connection();
        
        match SqlBank::find_by_id(&mut conn, bank_id) {
            Ok(bank) => Ok(Some(Bank {
                id: bank.id,
                name: bank.name,
                address: bank.address,
                phone: bank.phone,
                email: bank.email,
                created_at: chrono::DateTime::from_naive_utc_and_offset(bank.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(bank.updated_at, chrono::Utc),
            })),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(ServerFnError::new(format!("Failed to get bank: {}", e))),
        }
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only function"))
    }
} 