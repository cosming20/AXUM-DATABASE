use crate::db::schema::banks;
use crate::app::models::Bank;
use diesel::prelude::*;
#[cfg(feature = "ssr")]
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = banks)]
pub struct SqlBank {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = banks)]
pub struct NewBank {
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[cfg(feature = "ssr")]
impl SqlBank {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<SqlBank>, diesel::result::Error> {
        banks::table.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, bank_id: i32) -> Result<SqlBank, diesel::result::Error> {
        banks::table.filter(banks::id.eq(bank_id)).first(conn)
    }

    pub fn create_bank(
        conn: &mut PgConnection,
        name: String,
        address: String,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<SqlBank, diesel::result::Error> {
        let new_bank = NewBank {
            name,
            address,
            phone,
            email,
        };

        diesel::insert_into(banks::table)
            .values(&new_bank)
            .execute(conn)?;

        banks::table.order(banks::id.desc()).first(conn)
    }
}

impl SqlBank {
    pub fn get_bank_by_id(
        conn: &mut PgConnection,
        bank_id: i32,
    ) -> Result<SqlBank, diesel::result::Error> {
        banks::table
            .filter(banks::id.eq(bank_id))
            .first(conn)
    }

    pub fn get_all_banks(
        conn: &mut PgConnection,
    ) -> Result<Vec<SqlBank>, diesel::result::Error> {
        banks::table.load(conn)
    }

    pub fn update_bank(
        conn: &mut PgConnection,
        bank_id: i32,
        name: Option<String>,
        address: Option<String>,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<(), diesel::result::Error> {
        let now = Utc::now().naive_utc();
        
        if let Some(bank_name) = name {
            diesel::update(banks::table.filter(banks::id.eq(bank_id)))
                .set((banks::name.eq(bank_name), banks::updated_at.eq(now)))
                .execute(conn)?;
        }
        
        if let Some(bank_address) = address {
            diesel::update(banks::table.filter(banks::id.eq(bank_id)))
                .set((banks::address.eq(bank_address), banks::updated_at.eq(now)))
                .execute(conn)?;
        }
        
        if let Some(bank_phone) = phone {
            diesel::update(banks::table.filter(banks::id.eq(bank_id)))
                .set((banks::phone.eq(bank_phone), banks::updated_at.eq(now)))
                .execute(conn)?;
        }
        
        if let Some(bank_email) = email {
            diesel::update(banks::table.filter(banks::id.eq(bank_id)))
                .set((banks::email.eq(bank_email), banks::updated_at.eq(now)))
                .execute(conn)?;
        }

        Ok(())
    }

    pub fn delete_bank(
        conn: &mut PgConnection,
        bank_id: i32,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(banks::table.filter(banks::id.eq(bank_id)))
            .execute(conn)?;
        Ok(())
    }

    pub fn to_app_model(self) -> Bank {
        Bank {
            id: self.id,
            name: self.name,
            address: self.address,
            phone: self.phone,
            email: self.email,
            created_at: DateTime::from_naive_utc_and_offset(self.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(self.updated_at, Utc),
        }
    }

    pub fn to_app_models(banks: Vec<SqlBank>) -> Vec<Bank> {
        banks.into_iter().map(|b| b.to_app_model()).collect()
    }
} 