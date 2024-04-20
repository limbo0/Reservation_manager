use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::resv)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reservation {
    pub name: String,
    pub contact: String,
    pub seating: String,
    pub advance: bool,
    pub confirmed: bool,
}

use crate::schema::resv;

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = resv)]
pub struct NewResv {
    pub name: String,
    pub contact: String,
    pub seating: String,
    pub advance: bool,
    pub confirmed: bool,
}
