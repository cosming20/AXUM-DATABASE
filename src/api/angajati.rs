use crate::app;
use crate::db::models::*;
use leptos::prelude::ServerFnError;
use leptos::*;

#[server]
pub async fn create_angajati(
    nume: String,
    prenume: String,
    telefon: String,
    banca_id: i32,
) -> Result<SqlAngajat, ServerFnError> {
    use crate::db::models::SqlAngajat;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlAngajat::create_angajat(&mut conn, nume, prenume, telefon, banca_id) {
        Ok(angajat) => {
            // Successfully created angajat
            println!("Angajat created: {:?}", angajat);
            Ok(angajat) // Return the created angajat in the Ok variant
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error creating angajat: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error creating angajat: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}

#[server]
pub async fn get_angajati() -> Result<Vec<app::models::Angajat>, ServerFnError> {
    use crate::db::models::SqlAngajat;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlAngajat::get_all_angajati(&mut conn) {
        Ok(angajati) => {
            // Successfully retrieved angajati
            println!("Angajati retrieved: {:?}", angajati);
            let angajat_app = SqlAngajat::to_app_models(angajati);
            println!("Angajati  modelat retrieved: {:?}", angahat_app);
            Ok(angajat_app) // Return the retrieved angajati in the Ok variant
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error retrieving angajati: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error retrieving angajati: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}
