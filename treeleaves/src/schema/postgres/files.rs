// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> BigInt,
        name -> Varchar,
        path -> Varchar,
        mime_type_id -> Int4,
        size -> BigInt,
    }
}
