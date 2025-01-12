use crate::db::schema::*;
use diesel::result::Error as DieselError;
use diesel::{prelude::*, MysqlConnection};
use serde::{Deserialize, Serialize};
use crate::app::*;

#[derive(
    Serialize, Deserialize, Debug, Clone, Queryable, Identifiable, Selectable, AsChangeset,
)]
#[diesel(table_name = banca)]
pub struct SqlBanca {
    pub id: i32,
    pub nume: String,
    pub adresa: String,
}

#[derive(Insertable)]
#[diesel(table_name = banca)]
pub struct NewBanca {
    pub nume: String,
    pub adresa: String,
}

impl SqlBanca {
    
    pub fn create_banca(
        conn: &mut MysqlConnection, // Accept MySQL connection here (synchronous)
        nume: String,
        adresa: String,
    ) -> Result<Self, DieselError> {
        let new_banca = NewBanca {
            nume,
            adresa,
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

    pub fn to_app_model(&self) -> models::Banca {
        models::Banca {
            id: self.id,
            nume: self.nume.clone(),
            adresa: self.adresa.clone(),
        }
    }

    pub fn to_app_models(banci: Vec<Self>) -> Vec<models::Banca> {
        banci
            .into_iter()
            .map(|banca| banca.to_app_model())
            .collect()
    }

    pub fn get_banca_by_id(conn: &mut MysqlConnection, banca_id: i32) -> Result<Self, DieselError> {
        let banca = banca::table
            .filter(banca::id.eq(banca_id))
            .first::<SqlBanca>(conn)?;

        Ok(banca)
    }

    pub fn get_id_by_banca(conn: &mut MysqlConnection, banca_nume: String) -> Result<i32, DieselError> {
        let banca = banca::table
            .filter(banca::nume.eq(banca_nume))
            .select(banca::id)
            .first::<i32>(conn)?;

        Ok(banca)
    }

    pub fn delete_banca(conn: &mut MysqlConnection, banca_id: i32) -> Result<(), DieselError> {
        let deleted_rows = diesel::delete(banca::table.filter(banca::id.eq(banca_id)))
            .execute(conn)?;

        if deleted_rows == 0 {
            Err(DieselError::NotFound) // Return an error if no rows were deleted
        } else {
            Ok(()) // Return Ok variant indicating success
        }
    }

    pub fn edit_banca(
        conn: &mut MysqlConnection, 
        banca_id: i32,
        nume: Option<String>,
        adresa: Option<String>,
    ) -> Result<Self, DieselError> {
         // First, fetch the existing record to update
         let mut banca = banca::table
         .filter(banca::id.eq(banca_id))
         .first::<SqlBanca>(conn)?;

     // Conditionally update fields if they are Some(value)
     if let Some(nume_value) = nume {
         banca.nume = nume_value;
     }
     if let Some(adresa_value) = adresa {
         banca.adresa = adresa_value;
     }
     // Now, update the database with the modified values
     diesel::update(banca::table.filter(banca::id.eq(banca_id)))
         .set((
             banca::nume.eq(&banca.nume),
             banca::adresa.eq(&banca.adresa),
         ))
         .execute(conn)?;

     Ok(banca)
    }
}
