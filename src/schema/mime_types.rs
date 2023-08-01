// @generated automatically by Diesel CLI.

diesel::table! {
    mime_types (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Text,
    }
}
