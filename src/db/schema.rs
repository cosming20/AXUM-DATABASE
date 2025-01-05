// @generated automatically by Diesel CLI.

diesel::table! {
    angajati (id) {
        id -> Integer,
        #[max_length = 255]
        nume -> Varchar,
        #[max_length = 255]
        prenume -> Varchar,
        #[max_length = 255]
        telefon -> Varchar,
        banca_id -> Integer,
    }
}

diesel::table! {
    banca (id) {
        id -> Integer,
        #[max_length = 255]
        nume -> Varchar,
        #[max_length = 255]
        adresa -> Varchar,
    }
}

diesel::table! {
    sucursala (id) {
        id -> Integer,
        #[max_length = 200]
        nume -> Varchar,
        #[max_length = 200]
        adresa -> Varchar,
        banca_id -> Integer,
    }
}

diesel::joinable!(angajati -> banca (banca_id));
diesel::joinable!(sucursala -> banca (banca_id));

diesel::allow_tables_to_appear_in_same_query!(
    angajati,
    banca,
    sucursala,
);
