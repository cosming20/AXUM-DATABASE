use leptos::*;
use leptos::prelude::ServerFnError;
use crate::db::models::*;

#[server]
pub async fn create_banca(
    nume: String, adresa: String, sucursala_id: i32
) -> Result<SqlBanca, ServerFnError> {
    use crate::establish_connection;
    use crate::db::models::SqlBanca;

    let mut conn = establish_connection();

    match SqlBanca::create_banca(&mut conn, nume, adresa, sucursala_id) {
        Ok(banca) => {
            // Successfully created banca
            println!("Banca created: {:?}", banca);
            Ok(banca) // Return the created banca in the Ok variant
        },
        Err(e) => {
            // Handle the error
            eprintln!("Error creating banca: {:?}", e);
            Err(ServerFnError::new(format!("Error creating banca: {:?}", e))) // Return the error in the Err variant
        }
    }
}

#[server]
pub async fn get_banci() -> Result<Vec<SqlBanca>, ServerFnError> {
    use crate::establish_connection;
    use crate::db::models::SqlBanca;

    let mut conn = establish_connection();

    match SqlBanca::get_all_banci(&mut conn) {
        Ok(banci) => {
            // Successfully retrieved banci
            println!("Banci retrieved: {:?}", banci);
            Ok(banci) // Return the retrieved banci in the Ok variant
        },
        Err(e) => {
            // Handle the error
            eprintln!("Error retrieving banci: {:?}", e);
            Err(ServerFnError::new(format!("Error retrieving banci: {:?}", e))) // Return the error in the Err variant
        }
    }
}
