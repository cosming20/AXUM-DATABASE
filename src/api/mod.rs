pub mod auth;
pub mod users;
pub mod accounts;
pub mod transactions;
pub mod banks;
pub mod branches;

pub use auth::*;
pub use users::*;
pub use accounts::*;
pub use transactions::*;
pub use banks::*;
pub use branches::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientError {
    NotFound,
    NotAuthenticated,
    NoPrimaryEmail,
    NotVerified,
    NoPermission,
    InvalidCredentials,
    InsufficientFunds,
    AccountNotFound,
    TransactionFailed,
}
