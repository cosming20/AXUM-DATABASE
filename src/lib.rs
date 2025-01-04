pub mod api;
pub mod app;
pub mod db;
use app::pibd::App;
#[macro_use]
extern crate diesel;
extern crate dotenv;
// pub mod schema;
// pub mod models;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // use crate::pibd::src::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
