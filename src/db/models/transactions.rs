use crate::db::schema::transactions;
use crate::app::models::{Transaction, TransactionType, TransactionStatus, TransferRequest};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
#[cfg(feature = "ssr")]
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = transactions)]
pub struct SqlTransaction {
    pub id: i32,
    pub uuid: String,
    pub from_account_id: Option<i32>,
    pub to_account_id: Option<i32>,
    pub amount: BigDecimal,
    pub currency: String,
    pub transaction_type: String,
    pub description: Option<String>,
    pub status: String,
    pub reference_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub uuid: String,
    pub from_account_id: Option<i32>,
    pub to_account_id: Option<i32>,
    pub amount: BigDecimal,
    pub currency: String,
    pub transaction_type: String,
    pub description: Option<String>,
    pub status: String,
    pub reference_number: Option<String>,
}

impl SqlTransaction {
    pub fn create_deposit(
        conn: &mut PgConnection,
        account_id: i32,
        amount: f64,
        description: Option<String>,
    ) -> Result<SqlTransaction, DieselError> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let reference_number = format!("DEP-{}", chrono::Utc::now().timestamp());
        
        let new_transaction = NewTransaction {
            uuid,
            from_account_id: None,
            to_account_id: Some(account_id),
            amount: BigDecimal::from_str(&amount.to_string()).unwrap_or_default(),
            currency: "USD".to_string(),
            transaction_type: "deposit".to_string(),
            description,
            status: "completed".to_string(),
            reference_number: Some(reference_number),
        };

        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(conn)?;

        transactions::table
            .order(transactions::id.desc())
            .first(conn)
    }

    pub fn create_withdrawal(
        conn: &mut PgConnection,
        account_id: i32,
        amount: f64,
        description: Option<String>,
    ) -> Result<SqlTransaction, DieselError> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let reference_number = format!("WTH-{}", chrono::Utc::now().timestamp());
        
        let new_transaction = NewTransaction {
            uuid,
            from_account_id: Some(account_id),
            to_account_id: None,
            amount: BigDecimal::from_str(&amount.to_string()).unwrap_or_default(),
            currency: "USD".to_string(),
            transaction_type: "withdrawal".to_string(),
            description,
            status: "completed".to_string(),
            reference_number: Some(reference_number),
        };

        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(conn)?;

        transactions::table
            .order(transactions::id.desc())
            .first(conn)
    }

    pub fn create_transfer(
        conn: &mut PgConnection,
        from_account_id: i32,
        to_account_id: i32,
        amount: f64,
        description: Option<String>,
    ) -> Result<SqlTransaction, DieselError> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let reference_number = format!("TRF-{}", chrono::Utc::now().timestamp());
        
        let new_transaction = NewTransaction {
            uuid,
            from_account_id: Some(from_account_id),
            to_account_id: Some(to_account_id),
            amount: BigDecimal::from_str(&amount.to_string()).unwrap_or_default(),
            currency: "USD".to_string(),
            transaction_type: "transfer".to_string(),
            description,
            status: "completed".to_string(),
            reference_number: Some(reference_number),
        };

        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(conn)?;

        transactions::table
            .order(transactions::id.desc())
            .first(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, transaction_id: i32) -> Result<SqlTransaction, DieselError> {
        transactions::table
            .filter(transactions::id.eq(transaction_id))
            .first(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<SqlTransaction>, DieselError> {
        transactions::table
            .order(transactions::created_at.desc())
            .load(conn)
    }

    pub fn find_by_account(conn: &mut PgConnection, account_id: i32) -> Result<Vec<SqlTransaction>, DieselError> {
        transactions::table
            .filter(
                transactions::from_account_id.eq(account_id)
                .or(transactions::to_account_id.eq(account_id))
            )
            .order(transactions::created_at.desc())
            .load(conn)
    }

    pub fn update_status(
        conn: &mut PgConnection,
        transaction_id: i32,
        new_status: String,
    ) -> Result<usize, DieselError> {
        let now = chrono::Utc::now().naive_utc();
        
        diesel::update(transactions::table.filter(transactions::id.eq(transaction_id)))
            .set((
                transactions::status.eq(new_status),
                transactions::updated_at.eq(now)
            ))
            .execute(conn)
    }
} 