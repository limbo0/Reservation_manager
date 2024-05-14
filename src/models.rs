use axum::response::{Html, IntoResponse, Response};
use chrono::NaiveDate;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, Output, ToSql},
    sql_types::{self, Integer, Jsonb},
    Insertable, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use time::Time;
use uuid::Uuid;

#[repr(i32)]
#[derive(Debug, Serialize, Deserialize, Clone, AsExpression)]
#[diesel(sql_type = Integer)]
pub enum PaymentMode {
    NotPaid = 1,
    Cash = 2,
    Card = 3,
    Gpay = 4,
}

impl<DB> ToSql<Integer, DB> for PaymentMode
where
    DB: diesel::backend::Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        match self {
            PaymentMode::NotPaid => 0.to_sql(out),
            PaymentMode::Cash => 1.to_sql(out),
            PaymentMode::Card => 2.to_sql(out),
            PaymentMode::Gpay => 3.to_sql(out),
        }
    }
}

impl<DB> FromSql<Integer, DB> for PaymentMode
where
    DB: diesel::backend::Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(PaymentMode::NotPaid),
            1 => Ok(PaymentMode::Cash),
            2 => Ok(PaymentMode::Card),
            3 => Ok(PaymentMode::Gpay),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

//NOTE: change mode of  payment.
/// AsExpression is converting this struct to sql type Jsonb, which is the data type in our db.
#[derive(AsExpression, Queryable, Debug, Clone, Serialize, Deserialize)]
#[diesel(sql_type = Jsonb)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaymentMethod {
    pub mode_of_payment: PaymentMode,
    pub payment_transaction_id: String,
    pub payment_receiver: String,
    pub payment_received_date: NaiveDate,
}

impl FromSqlRow<Jsonb, diesel::pg::Pg> for PaymentMethod {
    fn build_from_row<'a>(
        row: &impl diesel::row::Row<'a, diesel::pg::Pg>,
    ) -> deserialize::Result<Self> {
        Ok(PaymentMethod {
            mode_of_payment: row.get_value(0)?,
            payment_transaction_id: row.get_value(1)?,
            payment_receiver: row.get_value(2)?,
            payment_received_date: row.get_value(3)?,
        })
    }
}

//FromSqlRow<diesel::sql_types::Jsonb, Pg>`
#[derive(Clone, Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::reservation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reservation {
    pub id: Uuid,
    pub name: String,
    pub contact: String,
    pub seating: String,
    pub specific_seating_requested: bool,
    pub advance: bool,
    pub advance_method: serde_json::Value,
    pub advance_amount: Option<i32>,
    pub confirmed: bool,
    pub reservation_date: NaiveDate,
    pub reservation_time: Time,
}

//TODO: not too sure about this.
// impl IntoResponse for Reservation {
//     fn into_response(self) -> Response {
//         let html = format!("id: {:?}, name: {:?}, contact: {:?}, seating: {:?}, specific_seating_requested: {:?}, advance: {:?}, advance_amount: {:?}, advance_method: {:?}, confirmed: {:?}, reservation_date: {:?}, reservation_time: {:?}", self.id, self.name, self.contact, self.seating, self.specific_seating_requested, self.advance, self.advance_amount, self.advance_method, self.confirmed, self.reservation_date, self.reservation_time);
//         Html(html).into_response()
//     }
// }
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
    pub advance_method: serde_json::Value,
    pub advance_amount: Option<i32>,
    pub confirmed: bool,
    pub reservation_date: NaiveDate,
    pub reservation_time: Time,
}
