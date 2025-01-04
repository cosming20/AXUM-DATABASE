pub mod angajati;
pub mod banca;
pub mod sucursala;

pub use angajati::*;
pub use banca::*;
pub use sucursala::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientError {
    NotFound,
    NotAuthenticated,
    NoPrimaryEmail,
    NotVerified,
    NoPermission,
}
