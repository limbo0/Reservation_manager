// @generated automatically by Diesel CLI.

diesel::table! {
    reservation (id) {
        id -> Int4,
        name -> Varchar,
        contact -> Text,
        seating -> Varchar,
        specific_seating_requested -> Bool,
        advance -> Bool,
        advance_method -> Nullable<Varchar>,
        advance_amount -> Nullable<Int4>,
        confirmed -> Bool,
        reservation_date -> Date,
        reservation_time -> Time,
    }
}

// diesel::table! {
//     resv (id) {
//         id -> Int4,
//         name -> Varchar,
//         contact -> Text,
//         seating -> Varchar,
//         advance -> Bool,
//         confirmed -> Bool,
//         reservation_date -> Date,
//     }
// }
//
// diesel::allow_tables_to_appear_in_same_query!(reservation, resv,);
