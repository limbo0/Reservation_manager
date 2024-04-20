// @generated automatically by Diesel CLI.

diesel::table! {
    resv (id) {
        id -> Int4,
        name -> Varchar,
        contact -> Text,
        seating -> Varchar,
        advance -> Bool,
        confirmed -> Bool,
    }
}
