use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Angajat {
    pub nume: String,
    pub prenume: String,
    pub telefon: String,
    pub banca_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Banca {
    pub nume: String,
    pub adresa: String,
    pub sucursala: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sucursala {
    pub nume: String,
    pub adresa: String,
}
