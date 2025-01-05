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
    pub nume: Option<String>,
    pub adresa: Option<String>,
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
        sucursala_id: i32,
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
            nume: self.nume.clone().unwrap_or_default(),
            adresa: self.adresa.clone().unwrap_or_default(),
        }
    }

    pub fn to_app_models(banci: Vec<Self>) -> Vec<models::Banca> {
        banci
            .into_iter()
            .map(|banca| banca.to_app_model())
            .collect()
    }
}
