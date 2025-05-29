use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Customer,
    Staff,
    Admin,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Customer => write!(f, "customer"),
            UserRole::Staff => write!(f, "staff"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "staff" => UserRole::Staff,
            "admin" => UserRole::Admin,
            _ => UserRole::Customer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bank {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Branch {
    pub id: i32,
    pub bank_id: i32,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub branch_code: String,
    pub bank_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub uuid: String,
    pub user_id: i32,
    pub branch_id: i32,
    pub account_number: String,
    pub account_type: AccountType,
    pub balance: Decimal,
    pub currency: String,
    pub is_active: bool,
    pub user_name: Option<String>,
    pub branch_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
    Business,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Checking => write!(f, "checking"),
            AccountType::Savings => write!(f, "savings"),
            AccountType::Credit => write!(f, "credit"),
            AccountType::Business => write!(f, "business"),
        }
    }
}

impl From<String> for AccountType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "savings" => AccountType::Savings,
            "credit" => AccountType::Credit,
            "business" => AccountType::Business,
            _ => AccountType::Checking,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub uuid: String,
    pub from_account_id: Option<i32>,
    pub to_account_id: Option<i32>,
    pub amount: Decimal,
    pub currency: String,
    pub transaction_type: TransactionType,
    pub description: Option<String>,
    pub status: TransactionStatus,
    pub reference_number: Option<String>,
    pub from_account_number: Option<String>,
    pub to_account_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer,
    Deposit,
    Withdrawal,
    Payment,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Transfer => write!(f, "transfer"),
            TransactionType::Deposit => write!(f, "deposit"),
            TransactionType::Withdrawal => write!(f, "withdrawal"),
            TransactionType::Payment => write!(f, "payment"),
        }
    }
}

impl From<String> for TransactionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "deposit" => TransactionType::Deposit,
            "withdrawal" => TransactionType::Withdrawal,
            "payment" => TransactionType::Payment,
            _ => TransactionType::Transfer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "pending"),
            TransactionStatus::Completed => write!(f, "completed"),
            TransactionStatus::Failed => write!(f, "failed"),
            TransactionStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl From<String> for TransactionStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "completed" => TransactionStatus::Completed,
            "failed" => TransactionStatus::Failed,
            "cancelled" => TransactionStatus::Cancelled,
            _ => TransactionStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from_account_id: i32,
    pub to_account_number: String,
    pub amount: Decimal,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub user_id: i32,
    pub account_type: AccountType,
    pub initial_balance: Decimal,
}
