// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
        mime_type_id -> Integer,
        size -> Integer,
    }
}
