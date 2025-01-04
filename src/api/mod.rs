pub mod angajati;
pub mod traits;
pub use angajati::*;
pub use traits::*;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientError {
    NotFound,
    NotAuthenticated,
    NoPrimaryEmail,
    NotVerified,
    NoPermission,
}