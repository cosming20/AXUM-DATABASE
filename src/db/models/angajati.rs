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

    pub fn to_app_model(&self) -> models::Angajat {
        models::Angajat {
            nume: self.nume.clone().unwrap_or_default(),
            prenume: self.prenume.clone().unwrap_or_default(),
            telefon: self.telefon.clone().unwrap_or_default(),
            banca_id: self.banca_id.unwrap_or(0),
        }
    }

    pub fn to_app_models(angajati: Vec<Self>) -> Vec<models::Angajat> {
        angajati
            .into_iter()
            .map(|angajat| angajat.to_app_model())
            .collect()
    }
}
