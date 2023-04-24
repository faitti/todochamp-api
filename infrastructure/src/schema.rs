// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}
