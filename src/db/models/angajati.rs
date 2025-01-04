use diesel::{prelude::*, MysqlConnection};
use serde::{Deserialize, Serialize};
use crate::db::schema::*;
use diesel::result::Error as DieselError;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = angajati)]
pub struct SqlAngajat {
    pub id: i32,
    pub nume: Option<String>,
    pub prenume: Option<String>,
    pub telefon: Option<String>,
    pub banca_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = angajati)]
struct NewAngajat {
    pub nume: String,
    pub prenume: String,
    pub telefon: String,
    pub banca_id: i32,
}

impl SqlAngajat {
    pub fn create_angajat(
        conn: &mut MysqlConnection,  // Accept MySQL connection here (synchronous)
        nume: String, 
        prenume: String, 
        telefon: String, 
        banca_id: i32
    ) -> Result<Self, DieselError> {
        let new_user = NewAngajat { nume, prenume, telefon, banca_id };
    
        diesel::insert_into(angajati::table)
        .values(&new_user)
        .execute(conn)?;

        let inserted_angajat = angajati::table
        .order(angajati::id.desc()) // Assuming id is auto-incremented
        .first(conn)?; // Get the last inserted angajat

        Ok(inserted_angajat)
    }
}