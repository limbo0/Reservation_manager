// @generated automatically by Diesel CLI.

diesel::table! {
    reservation (id) {
        id -> Uuid,
        name -> Varchar,
        contact -> Text,
        seating -> Varchar,
        specific_seating_requested -> Bool,
        advance -> Bool,
        advance_method -> Jsonb,
        advance_amount -> Nullable<Int4>,
        confirmed -> Bool,
        reservation_date -> Date,
        reservation_time -> Time,
    }
}
