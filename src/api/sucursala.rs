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
    
    // let banca_id = get_banca_id_by_nume(banca_nume);  



    // match SqlSucursala::create_sucursala(&mut conn, nume, adresa, banca_id) {
    //     Ok(sucursala) => {
    //         // Successfully created sucursala
    //         println!("Sucursala created: {:?}", sucursala);
    //         Ok(sucursala) // Return the created sucursala in the Ok variant
    //     }
    //     Err(e) => {
    //         // Handle the error
    //         eprintln!("Error creating sucursala: {:?}", e);
    //         Err(ServerFnError::new(format!(
    //             "Error creating sucursala: {:?}",
    //             e
    //         ))) // Return the error in the Err variant
    //     }
    // }

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
