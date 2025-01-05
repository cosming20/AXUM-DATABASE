use crate::app;
use crate::db::models::*;
use leptos::prelude::ServerFnError;
use leptos::*;

#[server]
pub async fn create_angajat(
    nume: String,
    prenume: String,
    telefon: String,
    banca_nume: String,
) -> Result<SqlAngajat, ServerFnError> {
    use crate::db::models::SqlAngajat;
    use crate::establish_connection;

    let mut conn = establish_connection();

    // match SqlAngajat::create_angajat(&mut conn, nume, prenume, telefon, banca_id) {
    //     Ok(angajat) => {
    //         // Successfully created angajat
    //         println!("Angajat created: {:?}", angajat);
    //         Ok(angajat) // Return the created angajat in the Ok variant
    //     }
    //     Err(e) => {
    //         // Handle the error
    //         eprintln!("Error creating angajat: {:?}", e);
    //         Err(ServerFnError::new(format!(
    //             "Error creating angajat: {:?}",
    //             e
    //         ))) // Return the error in the Err variant
    //     }
    // }
    use crate::api::banca::*;
    match get_banca_id_by_nume(banca_nume).await {
        Ok(banca_id) => {
            match SqlAngajat::create_angajat(&mut conn, nume, prenume, telefon, banca_id) {
                Ok(angajat) => {
                    // Successfully created sucursala
                    println!("Sucursala created: {:?}", angajat);
                    Ok(angajat) // Return the created sucursala in the Ok variant
                }
                Err(e) => {
                    // Handle the error
                    eprintln!("Error creating sucursala: {:?}", e);
                    Err(ServerFnError::new(format!(
                        "Error creating sucursala: {:?}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            // Handle the error fetching banca
            eprintln!("Error fetching banca by name: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error fetching banca by name: {:?}",
                e
            )))
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
            println!("Angajati  modelat retrieved: {:?}", angajat_app);
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
