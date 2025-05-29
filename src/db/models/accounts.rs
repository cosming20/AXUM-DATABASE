use crate::db::schema::accounts;
use crate::app::models::{Account, AccountType, CreateAccountRequest};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
#[cfg(feature = "ssr")]
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc, NaiveDateTime};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct SqlAccount {
    pub id: i32,
    pub uuid: String,
    pub user_id: i32,
    pub branch_id: i32,
    pub account_number: String,
    pub account_type: String,
    pub balance: BigDecimal,
    pub currency: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub uuid: String,
    pub user_id: i32,
    pub branch_id: i32,
    pub account_number: String,
    pub account_type: String,
    pub balance: BigDecimal,
    pub currency: String,
    pub is_active: bool,
}

impl SqlAccount {
    pub fn create_account(
        conn: &mut PgConnection,
        user_id: i32,
        branch_id: i32,
        account_type: String,
        initial_balance: Option<f64>,
    ) -> Result<SqlAccount, DieselError> {
        let account_number = Self::generate_account_number();
        let balance = BigDecimal::from_str(&initial_balance.unwrap_or(0.0).to_string()).unwrap_or_default();
        
        let new_account = NewAccount {
            uuid: uuid::Uuid::new_v4().to_string(),
            user_id,
            branch_id,
            account_number,
            account_type,
            balance,
            currency: "USD".to_string(),
            is_active: true,
        };

        diesel::insert_into(accounts::table)
            .values(&new_account)
            .execute(conn)?;

        accounts::table
            .order(accounts::id.desc())
            .first(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        account_id: i32,
    ) -> Result<SqlAccount, DieselError> {
        accounts::table
            .filter(accounts::id.eq(account_id))
            .first(conn)
    }

    pub fn find_by_number(
        conn: &mut PgConnection,
        account_number: &str,
    ) -> Result<SqlAccount, DieselError> {
        accounts::table
            .filter(accounts::account_number.eq(account_number))
            .first(conn)
    }

    pub fn find_by_account_number(
        conn: &mut PgConnection,
        account_number: &str,
    ) -> Result<SqlAccount, DieselError> {
        Self::find_by_number(conn, account_number)
    }

    pub fn find_by_user(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<Vec<SqlAccount>, DieselError> {
        accounts::table
            .filter(accounts::user_id.eq(user_id))
            .filter(accounts::is_active.eq(true))
            .load(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
    ) -> Result<Vec<SqlAccount>, DieselError> {
        accounts::table
            .filter(accounts::is_active.eq(true))
            .load(conn)
    }

    pub fn update_balance(
        conn: &mut PgConnection,
        account_id: i32,
        new_balance: f64,
    ) -> Result<usize, DieselError> {
        let balance = BigDecimal::from_str(&new_balance.to_string()).unwrap_or_default();
        let now = chrono::Utc::now().naive_utc();
        
        diesel::update(accounts::table.filter(accounts::id.eq(account_id)))
            .set((accounts::balance.eq(balance), accounts::updated_at.eq(now)))
            .execute(conn)
    }

    pub fn deactivate_account(
        conn: &mut PgConnection,
        account_id: i32,
    ) -> Result<usize, DieselError> {
        let now = chrono::Utc::now().naive_utc();
        
        diesel::update(accounts::table.filter(accounts::id.eq(account_id)))
            .set((accounts::is_active.eq(false), accounts::updated_at.eq(now)))
            .execute(conn)
    }

    fn generate_account_number() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("ACC{:010}", timestamp % 10000000000)
    }
} 