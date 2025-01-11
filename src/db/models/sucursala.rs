use crate::app::models;
use crate::db::schema::*;
use diesel::result::Error as DieselError;
use diesel::{prelude::*, MysqlConnection};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Clone, Queryable, Identifiable, Selectable, AsChangeset,
)]
#[diesel(table_name = sucursala)]
pub struct SqlSucursala {
    pub id: i32,
    pub nume: String,
    pub adresa: String,
    pub banca_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = sucursala)]
pub struct NewSucursala {
    pub nume: String,
    pub adresa: String,
    pub banca_id: i32,
}

impl SqlSucursala {
    // Create a new sucursala
    pub fn create_sucursala(
        conn: &mut MysqlConnection, // Accept MySQL connection here (synchronous)
        nume: String,
        adresa: String,
        banca_id: i32,
    ) -> Result<Self, DieselError> {
        let new_sucursala = NewSucursala { nume, adresa, banca_id };

        diesel::insert_into(sucursala::table)
            .values(&new_sucursala)
            .execute(conn)?;

        let inserted_sucursala = sucursala::table
            .order(sucursala::id.desc()) // Assuming id is auto-incremented
            .first(conn)?; // Get the last inserted sucursala

        Ok(inserted_sucursala)
    }

    // Fetch all sucursale
    pub fn get_all_sucursale(conn: &mut MysqlConnection) -> Result<Vec<Self>, DieselError> {
        let sucursale_list = sucursala::table.load::<SqlSucursala>(conn)?; // Load all sucursale from the database

        Ok(sucursale_list)
    }

    pub fn to_app_model(&self, conn: &mut MysqlConnection) -> models::Sucursala {
        use diesel::prelude::*;

        let get_nume = sucursala::table
            .inner_join(banca::table)
            .filter(sucursala::banca_id.eq(self.banca_id))
            .select(banca::nume)
            .first::<String>(conn);

        let banca_nume = get_nume.unwrap_or(("NO bank").to_string());

        models::Sucursala {
            id: self.id,
            nume: self.nume.clone(),
            adresa: self.adresa.clone(),
            banca_id: self.banca_id,
            banca_nume: banca_nume.clone(),
        }
    }

    pub fn to_app_models(sucursale: Vec<Self>, conn: &mut MysqlConnection) -> Vec<models::Sucursala> {
        sucursale
            .into_iter()
            .map(|sucursala| sucursala.to_app_model(conn))
            .collect()
    }

    pub fn delete_sucursala(conn: &mut MysqlConnection, sucursala_id: i32) -> Result<(), DieselError> {
        let deleted_rows = diesel::delete(sucursala::table.filter(sucursala::id.eq(sucursala_id)))
            .execute(conn)?;

        if deleted_rows == 0 {
            Err(DieselError::NotFound) // Return an error if no rows were deleted
        } else {
            Ok(()) // Return Ok variant indicating success
        }

        
    }

    pub fn edit_sucursala(
        conn: &mut MysqlConnection, 
        sucursala_id: i32,
        nume: Option<String>,
        adresa: Option<String>,
        banca_id: i32,
    ) -> Result<Self, DieselError> {
         // First, fetch the existing record to update
         let mut sucursala = sucursala::table
         .filter(sucursala::id.eq(sucursala_id))
         .first::<SqlSucursala>(conn)?;

     // Conditionally update fields if they are Some(value)
     if let Some(nume_value) = nume {
         sucursala.nume = nume_value;
     }
     if let Some(adresa_value) = adresa {
         sucursala.adresa = adresa_value;
     }

     sucursala.banca_id = banca_id;


     // Now, update the database with the modified values
     diesel::update(sucursala::table.filter(sucursala::id.eq(sucursala_id)))
         .set((
             sucursala::nume.eq(&sucursala.nume),
             sucursala::adresa.eq(&sucursala.adresa),
             sucursala::banca_id.eq(&sucursala.banca_id),
         ))
         .execute(conn)?;

     // Return the updated angajat
     Ok(sucursala)
    }

    pub fn get_banca_id(conn: &mut MysqlConnection, angajat_id: i32)-> Result<i32, DieselError>{

        let angajat = sucursala::table
            .filter(sucursala::id.eq(angajat_id))
            .first::<SqlSucursala>(conn);

            match angajat {
                Ok(angajat) => Ok(angajat.banca_id), // If successful, return banca_id
                Err(e) => Err(e), // If there was an error, return it
            }
    }

}
