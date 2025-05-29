pub mod api;
pub mod app;
pub mod db;
pub mod auth;

// Import all server functions to ensure they are registered
#[cfg(feature = "ssr")]
use api::{
    auth::*,
    users::*,
    accounts::*,
    transactions::*,
    banks::*,
    branches::*,
};

#[cfg(feature = "ssr")]
extern crate diesel;
#[cfg(feature = "ssr")]
extern crate dotenv;

#[cfg(feature = "ssr")]
use diesel::pg::PgConnection;
#[cfg(feature = "ssr")]
use diesel::prelude::*;
#[cfg(feature = "ssr")]
use dotenv::dotenv;
#[cfg(feature = "ssr")]
use std::env;

#[cfg(feature = "ssr")]
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::bank_app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
