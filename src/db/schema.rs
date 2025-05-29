// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Varchar,
        user_id -> Int4,
        branch_id -> Int4,
        #[max_length = 20]
        account_number -> Varchar,
        #[max_length = 50]
        account_type -> Varchar,
        balance -> Numeric,
        #[max_length = 3]
        currency -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    banks (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    branches (id) {
        id -> Int4,
        bank_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 10]
        branch_code -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        token -> Varchar,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Varchar,
        from_account_id -> Nullable<Int4>,
        to_account_id -> Nullable<Int4>,
        amount -> Numeric,
        #[max_length = 3]
        currency -> Varchar,
        #[max_length = 50]
        transaction_type -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Varchar,
        #[max_length = 255]
        reference_number -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 50]
        role -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(accounts -> branches (branch_id));
diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(branches -> banks (bank_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    banks,
    branches,
    sessions,
    transactions,
    users,
);
