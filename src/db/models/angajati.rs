use crate::app::*;
use crate::db::schema::*;
use diesel::result::Error as DieselError;
use diesel::{prelude::*, MysqlConnection};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Clone, Queryable, Identifiable, Selectable, AsChangeset,
)]
#[diesel(table_name = angajati)]
pub struct SqlAngajat {
    pub id: i32,
    pub nume: String,
    pub prenume: String,
    pub telefon: String,
    pub banca_id: i32,
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
        conn: &mut MysqlConnection, // Accept MySQL connection here (synchronous)
        nume: String,
        prenume: String,
        telefon: String,
        banca_id: i32,
    ) -> Result<Self, DieselError> {
        let new_user = NewAngajat {
            nume,
            prenume,
            telefon,
            banca_id,
        };

        diesel::insert_into(angajati::table)
            .values(&new_user)
            .execute(conn)?;

        let inserted_angajat = angajati::table
            .order(angajati::id.desc()) // Assuming id is auto-incremented
            .first(conn)?; // Get the last inserted angajat

        Ok(inserted_angajat)
    }

    pub fn get_all_angajati(conn: &mut MysqlConnection) -> Result<Vec<Self>, DieselError> {
        let angajati_list = angajati::table.load::<SqlAngajat>(conn)?; // Load all angajati from the database

        Ok(angajati_list)
    }

    pub fn to_app_model(&self, conn: &mut MysqlConnection) -> models::Angajat {

        let get_nume = angajati::table
            .inner_join(banca::table)
            .filter(angajati::banca_id.eq(self.banca_id))
            .select(banca::nume)
            .first::<String>(conn);

        let banca_nume = get_nume.unwrap_or(("NO bank").to_string());

        models::Angajat {
            id: self.id,
            nume: self.nume.clone(),
            prenume: self.prenume.clone(),
            telefon: self.telefon.clone(),
            banca_id: self.banca_id,
            banca_nume: banca_nume.clone(),
        }
    }  

    pub fn to_app_models(angajati: Vec<Self>, conn: &mut MysqlConnection) -> Vec<models::Angajat> {
        angajati
            .into_iter()
            .map(|angajat| angajat.to_app_model(conn))
            .collect()
    }

    pub fn delete_angajat(conn: &mut MysqlConnection, angajat_id: i32) -> Result<(), DieselError> {
        let deleted_rows = diesel::delete(angajati::table.filter(angajati::id.eq(angajat_id)))
            .execute(conn)?;

        if deleted_rows == 0 {
            Err(DieselError::NotFound) // Return an error if no rows were deleted
        } else {
            Ok(()) // Return Ok variant indicating success
        }

        
    }

    pub fn edit_angajat(
        conn: &mut MysqlConnection, 
        angajat_id: i32,
        nume: Option<String>,
        prenume: Option<String>,
        banca_id: i32,
    ) -> Result<Self, DieselError> {
         // First, fetch the existing record to update
         let mut angajat = angajati::table
         .filter(angajati::id.eq(angajat_id))
         .first::<SqlAngajat>(conn)?;

     // Conditionally update fields if they are Some(value)
     if let Some(nume_value) = nume {
         angajat.nume = nume_value;
     }
     if let Some(prenume_value) = prenume {
         angajat.prenume = prenume_value;
     }

     angajat.banca_id = banca_id;


     // Now, update the database with the modified values
     diesel::update(angajati::table.filter(angajati::id.eq(angajat_id)))
         .set((
             angajati::nume.eq(&angajat.nume),
             angajati::prenume.eq(&angajat.prenume),
             angajati::banca_id.eq(&angajat.banca_id),
         ))
         .execute(conn)?;

     // Return the updated angajat
     Ok(angajat)
    }

    pub fn get_banca_id(conn: &mut MysqlConnection, angajat_id: i32)-> Result<i32, DieselError>{

        let angajat = angajati::table
            .filter(angajati::id.eq(angajat_id))
            .first::<SqlAngajat>(conn);

            match angajat {
                Ok(angajat) => Ok(angajat.banca_id), // If successful, return banca_id
                Err(e) => Err(e), // If there was an error, return it
            }
    }
}
