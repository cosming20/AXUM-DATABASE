use crate::db::schema::*;
use diesel::result::Error as DieselError;
use diesel::{prelude::*, MysqlConnection};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Clone, Queryable, Identifiable, Selectable, AsChangeset,
)]
#[diesel(table_name = banca)]
pub struct SqlBanca {
    pub id: i32,
    pub nume: Option<String>,
    pub adresa: Option<String>,
    pub sucursala_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = banca)]
pub struct NewBanca {
    pub nume: String,
    pub adresa: String,
    pub sucursala_id: i32,
}

impl SqlBanca {
    // Create a new banca
    pub fn create_banca(
        conn: &mut MysqlConnection, // Accept MySQL connection here (synchronous)
        nume: String,
        adresa: String,
        sucursala_id: i32,
    ) -> Result<Self, DieselError> {
        let new_banca = NewBanca {
            nume,
            adresa,
            sucursala_id,
        };

        diesel::insert_into(banca::table)
            .values(&new_banca)
            .execute(conn)?;

        let inserted_banca = banca::table
            .order(banca::id.desc()) // Assuming id is auto-incremented
            .first(conn)?; // Get the last inserted banca

        Ok(inserted_banca)
    }

    // Fetch all banci
    pub fn get_all_banci(conn: &mut MysqlConnection) -> Result<Vec<Self>, DieselError> {
        let banci_list = banca::table.load::<SqlBanca>(conn)?; // Load all banci from the database

        Ok(banci_list)
    }
}
