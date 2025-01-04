use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SqlAngajat {
    pub nume: String,
    pub prenume: String,
    pub telefon: String,
    pub banca_id: i32,
}

