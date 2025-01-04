use leptos::*;
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

#[server]
pub async fn get_angajati() -> Result<Vec<SqlAngajat>, ServerFnError> {
    use crate::establish_connection;
    use crate::db::models::SqlAngajat;

    let mut conn = establish_connection();

    match SqlAngajat::get_all_angajati(&mut conn) {
        Ok(angajati) => {
            // Successfully retrieved angajati
            println!("Angajati retrieved: {:?}", angajati);
            Ok(angajati) // Return the retrieved angajati in the Ok variant
        },
        Err(e) => {
            // Handle the error
            eprintln!("Error retrieving angajati: {:?}", e);
            Err(ServerFnError::new(format!("Error retrieving angajati: {:?}", e))) // Return the error in the Err variant
        }
    }
}


