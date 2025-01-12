use crate::app::models;
use crate::db::models::*;
use leptos::prelude::ServerFnError;
use leptos::*;

#[server]
pub async fn create_sucursala(nume: String, adresa: String, banca_nume: String) -> Result<SqlSucursala, ServerFnError> {
    use crate::db::models::SqlSucursala;
    use crate::establish_connection;

    let mut conn = establish_connection();

    use crate::api::banca::*;

    match get_banca_id_by_nume(banca_nume).await {
        Ok(banca_id) => {
            match SqlSucursala::create_sucursala(&mut conn, nume, adresa, banca_id) {
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
pub async fn get_sucursale() -> Result<Vec<models::Sucursala>, ServerFnError> {
    use crate::db::models::SqlSucursala;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlSucursala::get_all_sucursale(&mut conn) {
        Ok(sucursale) => {
            // Successfully retrieved sucursale
            println!("Sucursale retrieved: {:?}", sucursale);
            let sucursala_app = SqlSucursala::to_app_models(sucursale,&mut conn);
            Ok(sucursala_app) // Return the retrieved sucursale in the Ok variant
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

#[server]
pub async fn edit_sucursala(sucursala_id: i32, nume: Option<String>, adresa: Option<String>, banca: Option<String>) -> Result<(), ServerFnError> {
    use crate::db::models::SqlSucursala;
    use crate::establish_connection;

    let mut conn = establish_connection();

    use crate::api::banca::*;
    match banca {
        Some(banca_nume) =>  {
            match get_banca_id_by_nume(banca_nume).await {
                Ok(banca_id) => {
                match SqlSucursala::edit_sucursala(&mut conn, sucursala_id, nume, adresa, banca_id) {
                    Ok(_) => {
                        println!("Sucursala with ID {} edited successfully.", sucursala_id);
                        Ok(()) // Return Ok variant indicating success
                    }
                    Err(e) => {
                        // Handle the error
                        eprintln!("Error editing sucursala: {:?}", e);
                        Err(ServerFnError::new(format!(
                            "Error edit sucursala: {:?}",
                            e
                        ))) // Return the error in the Err variant
                    }
                }
            }
            Err(e) => {
                // Handle the error
                eprintln!("Error editing sucursala: {:?}", e);
                Err(ServerFnError::new(format!(
                    "Error edit sucursala: {:?}",
                    e
                ))) // Return the error in the Err variant
            }
        }
        
    }
        _ => {
            match SqlSucursala::get_banca_id(&mut conn, sucursala_id) {
                Ok(banca_id) => {
                    match SqlSucursala::edit_sucursala(&mut conn, sucursala_id, nume, adresa, banca_id) {
                        Ok(_) => {
                            println!("Sucursala with ID {} edited successfully.", sucursala_id);
                            Ok(()) // Return Ok variant indicating success
                        }
                        Err(e) => {
                            // Handle the error
                            eprintln!("Error editing sucursala: {:?}", e);
                            Err(ServerFnError::new(format!(
                                "Error edit sucursala: {:?}",
                                e
                            ))) // Return the error in the Err variant
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error getting banca id from sucursala: {:?}", e);
                        Err(ServerFnError::new(format!(
                            "Error getting banca id from sucursala: {:?}",
                            e
                        ))) // Return the error in the Err varia
                }
            }
        }
    }
}


#[server]
pub async fn delete_sucursala(sucursala_id: i32) -> Result<(), ServerFnError> {
    use crate::db::models::SqlSucursala;
    use crate::establish_connection;

    let mut conn = establish_connection();

    match SqlSucursala::delete_sucursala(&mut conn, sucursala_id) {
        Ok(_) => {
            println!("Sucursala with ID {} deleted successfully.", sucursala_id);
            Ok(()) // Return Ok variant indicating success
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error deleting sucursala: {:?}", e);
            Err(ServerFnError::new(format!(
                "Error deleting sucursala: {:?}",
                e
            ))) // Return the error in the Err variant
        }
    }
}

