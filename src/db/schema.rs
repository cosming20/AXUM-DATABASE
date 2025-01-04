// @generated automatically by Diesel CLI.

diesel::table! {
    angajati (id) {
        id -> Integer,
        #[max_length = 255]
        nume -> Nullable<Varchar>,
        #[max_length = 255]
        prenume -> Nullable<Varchar>,
        #[max_length = 255]
        telefon -> Nullable<Varchar>,
        banca_id -> Nullable<Integer>,
    }
}

diesel::table! {
    banca (id) {
        id -> Integer,
        #[max_length = 255]
        nume -> Nullable<Varchar>,
        #[max_length = 255]
        adresa -> Nullable<Varchar>,
        sucursala_id -> Nullable<Integer>,
    }
}

diesel::table! {
    sucursala (id) {
        id -> Integer,
        #[max_length = 200]
        nume -> Nullable<Varchar>,
        #[max_length = 200]
        adresa -> Nullable<Varchar>,
    }
}

diesel::joinable!(angajati -> banca (banca_id));
diesel::joinable!(banca -> sucursala (sucursala_id));

diesel::allow_tables_to_appear_in_same_query!(angajati, banca, sucursala,);
