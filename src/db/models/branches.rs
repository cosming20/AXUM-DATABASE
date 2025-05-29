use crate::db::schema::branches;
use crate::app::models::Branch;
use diesel::prelude::*;
#[cfg(feature = "ssr")]
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = branches)]
pub struct SqlBranch {
    pub id: i32,
    pub bank_id: i32,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub branch_code: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = branches)]
pub struct NewBranch {
    pub bank_id: i32,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub branch_code: String,
}

#[cfg(feature = "ssr")]
impl SqlBranch {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<SqlBranch>, diesel::result::Error> {
        branches::table.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, branch_id: i32) -> Result<SqlBranch, diesel::result::Error> {
        branches::table.filter(branches::id.eq(branch_id)).first(conn)
    }

    pub fn find_by_bank(conn: &mut PgConnection, bank_id: i32) -> Result<Vec<SqlBranch>, diesel::result::Error> {
        branches::table.filter(branches::bank_id.eq(bank_id)).load(conn)
    }

    pub fn create_branch(
        conn: &mut PgConnection,
        bank_id: i32,
        name: String,
        address: String,
        phone: Option<String>,
        branch_code: String,
    ) -> Result<SqlBranch, diesel::result::Error> {
        let new_branch = NewBranch {
            bank_id,
            name,
            address,
            phone,
            branch_code,
        };

        diesel::insert_into(branches::table)
            .values(&new_branch)
            .execute(conn)?;

        branches::table.order(branches::id.desc()).first(conn)
    }

    pub fn to_app_model(self) -> Branch {
        Branch {
            id: self.id,
            bank_id: self.bank_id,
            name: self.name,
            address: self.address,
            phone: self.phone,
            branch_code: self.branch_code,
            bank_name: None, // TODO: Join with banks table to get bank name
            created_at: DateTime::from_naive_utc_and_offset(self.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(self.updated_at, Utc),
        }
    }

    pub fn to_app_models(branches: Vec<SqlBranch>) -> Vec<Branch> {
        branches.into_iter().map(|b| b.to_app_model()).collect()
    }
} 