use axum::response::{Html, IntoResponse, Response};
use chrono::NaiveDate;
use diesel::{pg::PgConnection, prelude::*};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use time::Time;

#[derive(Clone, Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::reservation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reservation {
    pub id: i32,
    pub name: String,
    pub contact: String,
    pub seating: String,
    pub specific_seating_requested: bool,
    pub advance: bool,
    pub advance_amount: Option<i32>,
    pub advance_method: Option<String>,
    pub confirmed: bool,
    pub reservation_date: NaiveDate,
    pub reservation_time: Time,
}

//TODO: not too sure about this.
impl IntoResponse for Reservation {
    fn into_response(self) -> Response {
        let html = format!("id: {:?}, name: {:?}, contact: {:?}, seating: {:?}, specific_seating_requested: {:?}, advance: {:?}, advance_amount: {:?}, advance_method: {:?}, confirmed: {:?}, reservation_date: {:?}, reservation_time: {:?}", self.id, self.name, self.contact, self.seating, self.specific_seating_requested, self.advance, self.advance_amount, self.advance_method, self.confirmed, self.reservation_date, self.reservation_time);
        Html(html).into_response()
    }
}
impl Display for Reservation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {},{}, {:?}, {:?}, {}, {}, {} )",
            self.id,
            self.name,
            self.contact,
            self.seating,
            self.specific_seating_requested,
            self.advance,
            self.advance_amount,
            self.advance_method,
            self.confirmed,
            self.reservation_date,
            self.reservation_time,
        )
    }
}

use crate::schema::reservation;

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = reservation)]
pub struct NewResv {
    pub name: String,
    pub contact: String,
    pub seating: String,
    pub specific_seating_requested: bool,
    pub advance: bool,
    pub advance_method: Option<String>,
    pub advance_amount: Option<i32>,
    pub confirmed: bool,
    pub reservation_date: NaiveDate,
    pub reservation_time: Time,
}
