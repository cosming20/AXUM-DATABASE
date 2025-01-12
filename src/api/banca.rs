use crate::app;
use crate::db::models::*;
use leptos::prelude::ServerFnError;
use leptos::*;

#[server]
pub async fn create_banca(
    nume: String,
    adresa: String,
) -> Result<SqlBanca, ServerFnError> {
    use crate::db::models::SqlBanca;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlBanca::create_banca(&mut conn, nume, adresa) {
        Ok(banca) => {
            // Successfully created banca
            println!("Banca created: {:?}", banca);
            Ok(banca) // Return the created banca in the Ok variant
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error creating banca: {:?}", e);
            Err(ServerFnError::new(format!("Error creating banca: {:?}", e))) // Return the error in the Err variant
        }
    }
}

#[server]
pub async fn get_banci() -> Result<Vec<app::models::Banca>, ServerFnError> {
    use crate::db::models::SqlBanca;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlBanca::get_all_banci(&mut conn) {
        Ok(banci) => {
            // Successfully retrieved banci
            println!("Banci retrieved: {:?}", banci);
            let banci_app = SqlBanca::to_app_models(banci);
            Ok(banci_app) // Return the retrieved banci in the Ok variant
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error retrieving banci: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error retrieving banci: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}

#[server]
pub async fn get_banca_nume_by_id(banca_id: i32) -> Result<String, ServerFnError> {
    use crate::db::models::SqlBanca;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlBanca::get_banca_by_id(&mut conn, banca_id) {
        Ok(banca) => {
            // Successfully retrieved banca
            println!("Banca retrieved: {:?}", banca);
            Ok(banca.nume) // Return the name of the banca in the Ok variant
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error retrieving banca by id: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error retrieving banca by id: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}

#[server]
pub async fn get_banca_id_by_nume(nume: String) -> Result<i32, ServerFnError> {
    use crate::db::models::SqlBanca;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlBanca::get_id_by_banca(&mut conn, nume) {
        Ok(banca_id) => {
            // Successfully retrieved banca_id
            println!("Banca ID retrieved: {:?}", banca_id);
            Ok(banca_id) // Return the banca_id in the Ok variant
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error retrieving banca ID by nume: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error retrieving banca ID by nume: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}


#[server]
pub async fn edit_banca(banca_id: i32, nume: Option<String>, adresa: Option<String>) -> Result<(), ServerFnError> {
    use crate::db::models::SqlBanca;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlBanca::edit_banca(&mut conn, banca_id, nume, adresa) {
        Ok(_) => {
            println!("Banca with ID {} edited successfully.", banca_id);
            Ok(()) // Return Ok variant indicating success
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error editing banca: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error edit banca: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}


#[server]
pub async fn delete_banca(banca_id: i32) -> Result<(), ServerFnError> {
    use crate::db::models::SqlBanca;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlBanca::delete_banca(&mut conn, banca_id) {
        Ok(_) => {
            println!("Banca with ID {} deleted successfully.", banca_id);
            Ok(()) // Return Ok variant indicating success
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error deleting banca: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error deleting banca: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}

