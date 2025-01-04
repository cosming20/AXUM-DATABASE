// use leptos::server::ServerFnError;
// use serde::{Deserialize, Serialize};
// use std::fmt::Debug;

// /// Trait to provide context
// pub trait ContextProvider<T> {
//     fn provide(&self) -> T;
// }
// pub enum ClientError {
//     NotFound,
//     NotAuthenticated,
//     NoPrimaryEmail,
//     NotVerified,
//     NoPermission,
//     NoPermissionAdmin,
// }

// pub trait ServerFnErrorExt<T, E> {
//     /// Maps any error to a ServerFnError::ServerError with debug formatting
//     fn map_esrv(self) -> Result<T, ServerFnError<E>>;
// }

// impl<T, E, Err: Debug> ServerFnErrorExt<T, E> for Result<T, Err> {
//     fn map_esrv(self) -> Result<T, ServerFnError<E>> {
//         self.map_err(|e| ServerFnError::ServerError(format!("{:?}", e)))
//     }
// }

// #[cfg(feature = "ssr")]
// pub mod ssr {
//     use diesel_async::pg::AsyncPgConnection;
//     use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
//     use leptos::*;

//     use crate::app::models::*;
//     use crate::server::prelude::*;

//     pub fn db_pool() -> Result<DbPool, ServerFnError> {
//         use_context::<DbPool>().ok_or_else(|| ServerFnError::ServerError("DB Pool missing.".into()))
//     }

//     pub type DbPool = Pool<AsyncPgConnection>;
//     pub type DbConnection<'a> = PooledConnection<'a, AsyncPgConnection>;

//     /// Application context

//     pub struct AppContext {
//         pub db_pool: DbPool,
//     }

//     impl AppContext {
//         pub fn new<E>() -> Result<Self, ServerFnError<E>> {

//             let db_pool = db_pool().map_esrv()?;

//             Ok(Self { db_pool })
//         }
//     }
// }
