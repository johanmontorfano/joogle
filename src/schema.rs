// @generated automatically by Diesel CLI.

diesel::table! {
    domains (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        owned_by -> Uuid,
        domain -> Text,
    }
}
