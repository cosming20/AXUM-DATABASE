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
    pub nume: Option<String>,
    pub adresa: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = sucursala)]
pub struct NewSucursala {
    pub nume: String,
    pub adresa: String,
}

impl SqlSucursala {
    // Create a new sucursala
    pub fn create_sucursala(
        conn: &mut MysqlConnection, // Accept MySQL connection here (synchronous)
        nume: String,
        adresa: String,
    ) -> Result<Self, DieselError> {
        let new_sucursala = NewSucursala { nume, adresa };

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

    pub fn to_app_model(&self) -> models::Sucursala {
        models::Sucursala {
            nume: self.nume.clone().unwrap_or_default(),
            adresa: self.adresa.clone().unwrap_or_default(),
        }
    }

    pub fn to_app_models(sucursale: Vec<Self>) -> Vec<models::Sucursala> {
        sucursale
            .into_iter()
            .map(|sucursala| sucursala.to_app_model())
            .collect()
    }
}
