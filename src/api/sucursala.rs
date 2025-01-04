use crate::db::models::*;
use leptos::prelude::ServerFnError;
use leptos::*;

#[server]
pub async fn create_sucursala(nume: String, adresa: String) -> Result<SqlSucursala, ServerFnError> {
    use crate::db::models::SqlSucursala;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlSucursala::create_sucursala(&mut conn, nume, adresa) {
        Ok(sucursala) => {
            // Successfully created sucursala
            println!("Sucursala created: {:?}", sucursala);
            Ok(sucursala) // Return the created sucursala in the Ok variant
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error creating sucursala: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error creating sucursala: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}

#[server]
pub async fn get_sucursale() -> Result<Vec<SqlSucursala>, ServerFnError> {
    use crate::db::models::SqlSucursala;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlSucursala::get_all_sucursale(&mut conn) {
        Ok(sucursale) => {
            // Successfully retrieved sucursale
            println!("Sucursale retrieved: {:?}", sucursale);
            Ok(sucursale) // Return the retrieved sucursale in the Ok variant
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error retrieving sucursale: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error retrieving sucursale: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}
