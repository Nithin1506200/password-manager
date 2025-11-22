// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    profiles (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
        pass_hash -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    secrets (id) {
        id -> Text,
        profile_id -> Text,
        created_at -> Timestamp,
        data -> Text,
    }
}

diesel::joinable!(secrets -> profiles (profile_id));

diesel::allow_tables_to_appear_in_same_query!(profiles, secrets,);
