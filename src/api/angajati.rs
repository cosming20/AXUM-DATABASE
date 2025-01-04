use diesel::Connection;
use leptos::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::app;
use super::ClientError;
use leptos::prelude::ServerFnError;
use crate::db::models::*;

#[server]
pub async fn create_angajati(
    nume: String, prenume: String, telefon: String, banca_id: i32
) -> Result<SqlAngajat, ServerFnError> {
    use crate::establish_connection;
    use crate::db::models::SqlAngajat;
    
    let mut conn = establish_connection();

    match SqlAngajat::create_angajat(&mut conn, nume, prenume, telefon, banca_id) {
        Ok(angajat) => {
            // Successfully created angajat
            println!("Angajat created: {:?}", angajat);
            Ok(angajat) // Return the created angajat in the Ok variant
        },
        Err(e) => {
            // Handle the error
            eprintln!("Error creating angajat: {:?}", e);
            Err(ServerFnError::new(format!("Error creating angajat: {:?}", e))) // Return the error in the Err variant
        }
    }
}

// // #[server]
// // pub async fn get_angajati() -> Result<Vec<models::SqlAngajat>, ServerFnError> {
// //     use crate::api::traits::ssr::AppContext;
// //     let ctx = AppContext::new()?;
// //     let mut conn = ctx.db_pool.get().await.map_esrv()?;

// //     let angajati = models::SqlAngajat::get_all(&mut conn)
// //         .await
// //         .map_esrv()?;

// //     Ok(angajati)
// // }

