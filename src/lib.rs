use self::models::{NewResv, PaymentMethod, PaymentMode, Reservation};
use crate::schema::reservation;
use axum::{
    extract::{Json, Path, Query, State},
    response::{Html, IntoResponse},
};
use chrono::NaiveDate;

use diesel::prelude::*;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    r2d2::ConnectionManager,
    serialize::{Output, ToSql},
    AsExpression, PgConnection,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Test purpose insertion.
pub async fn insert_resv() {
    let data = NewResv {
        name: String::from("Boa Hancock"),
        contact: String::from("+91123456789"),
        seating: String::from("MainT7"),
        specific_seating_requested: true,
        advance: true,
        advance_amount: Some(5000i32),
        advance_method: serde_json::value::to_value(PaymentMethod {
            mode_of_payment: PaymentMode::Card,
            payment_transaction_id: String::from("0x1dac45"),
            payment_receiver: String::from("Limboo"),
            payment_received_date: NaiveDate::from_ymd_opt(2024i32, 05u32, 14u32)
                .expect("failed to strcuture date while testing insert post request"),
        })
        .expect("failed to convert payment method to json"),
        confirmed: true,
        reservation_date: NaiveDate::from_ymd_opt(2024i32, 05u32, 14u32)
            .expect("failed to strcuture date while testing insert post request"),
        reservation_time: time::Time::from_hms(14u8, 0u8, 0u8)
            .expect("failed to structure time for reservation."),
    };

    use self::schema::reservation::dsl::reservation;
    diesel::insert_into(reservation)
        .values(&data)
        .execute(&mut establish_connection())
        .expect("Error saving new resv");
}

//NOTE: CREATE
pub async fn create_resv(Json(payload): Json<NewResv>) {
    // build the data structure
    use self::schema::reservation::dsl::reservation;
    let new_resv = NewResv {
        name: payload.name,
        contact: payload.contact,
        seating: payload.seating,
        specific_seating_requested: payload.specific_seating_requested,
        advance: payload.advance,
        advance_method: payload.advance_method,
        advance_amount: payload.advance_amount,
        confirmed: payload.confirmed,
        reservation_date: payload.reservation_date,
        reservation_time: payload.reservation_time,
    };

    // actually insert the data.
    diesel::insert_into(reservation)
        .values(&new_resv)
        .execute(&mut establish_connection())
        .expect("Error saving new resv");
}

//NOTE: READ
//NOTE: Return with ascending order in time of upcoming reservations.
//TODO: Returning in Html, to implement Htmx (test and see)
/// Check the current date and return all reservations for that date.
#[axum_macros::debug_handler]
pub async fn read_resv_json() -> impl IntoResponse {
    use self::schema::reservation::dsl::{reservation, reservation_date};

    //TODO: Set it to Utc instead of local.
    // check the current date and return all reservations for that date.
    let results = reservation
        .filter(reservation_date.eq(chrono::offset::Local::now().date_naive()))
        .limit(5)
        .select(Reservation::as_select())
        .load(&mut establish_connection())
        .expect("Error loading resv");

    Json(results)
}

/// Returns reservation with matching date.
#[axum_macros::debug_handler]
pub async fn read_resv_with_date(Query(date): Query<MyDate>) -> Json<Vec<Reservation>> {
    use self::schema::reservation::dsl::{reservation, reservation_date};

    let resv_date = NaiveDate::from_ymd_opt(date.year, date.month, date.day)
        .expect("failed creating date for query");

    let results = reservation
        .filter(reservation_date.eq(resv_date))
        .limit(5)
        .select(Reservation::as_select())
        .load(&mut establish_connection())
        .expect("Error loading resv");
    // println!("match found: {:?}", results);

    Json(results)
}

/// Returns reservation with matching id.
//TODO: Handle while reservation for Id is not found.
pub async fn read_resv_with_id(Path(resv_id): Path<uuid::Uuid>) -> impl IntoResponse {
    use self::schema::reservation::dsl::{id, reservation};
    let results = reservation
        .filter(id.eq(resv_id))
        .select(Reservation::as_select())
        .load(&mut establish_connection())
        .expect("Couldn't find reservation with provided id.");

    // Since all the id's are unique it should always only return one value.
    if results.len() == 1 {
        Html(results[0].to_string())
    } else {
        Html(format!(
            "Couldn't find reservation with the provided id: {:?}",
            resv_id
        ))
    }
}

//NOTE: UPDATE
/// Checks if the reservation with the id exists, if true then update name.
pub async fn update_resv_name_with_id(Path((resv_id, new_name)): Path<(uuid::Uuid, String)>) {
    use self::schema::reservation::dsl::{id, name, reservation};

    diesel::update(reservation)
        .filter(id.eq(resv_id))
        .set(name.eq(new_name))
        .execute(&mut establish_connection())
        .expect(&format!(
            "Failed to update name for reservation Id: {:?}",
            resv_id
        ));
}

/// Checks if the reservation with the id exists, if true then update date.
pub async fn update_resv_date_with_id(Path((resv_id, new_date)): Path<(uuid::Uuid, NaiveDate)>) {
    use self::schema::reservation::dsl::{id, reservation, reservation_date};

    diesel::update(reservation)
        .filter(id.eq(resv_id))
        .set(reservation_date.eq(new_date))
        .execute(&mut establish_connection())
        .expect(&format!(
            "Failed to update date for reservation Id: {:?}",
            resv_id
        ));
}

///NOTE: DELETE
pub async fn delete_resv_with_id(Path(resv_id): Path<uuid::Uuid>) {
    use self::schema::reservation::dsl::{id, reservation};

    diesel::delete(reservation)
        .filter(id.eq(resv_id))
        .execute(&mut establish_connection())
        .expect(&format!(
            "Failed to delete reservation with Id: {:?}",
            resv_id
        ));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyDate {
    year: i32,
    month: u32,
    day: u32,
}
impl std::fmt::Display for MyDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_if_updates() -> anyhow::Result<()> {
        let client = reqwest::Client::new();

        let url = "http://127.0.0.1:3000/update_resv_with_id";

        let response = client.get(url).send().await?.text().await?;
        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn test_id_resv_exist() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let resv_id = 1;

        let url = format!("http://127.0.0.1:3000/update_resv/{}", resv_id);

        let response = client.get(url).send().await?.text().await?;

        // Need to wrap Reservation in vector since the query returns a vec of reservation.
        let response: Vec<Reservation> = serde_json::from_str(&response)?;

        println!("{:#?}", response);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_resv() -> anyhow::Result<()> {
        let date = MyDate {
            year: 2024i32,
            month: 05u32,
            day: 25u32,
        };

        let client = reqwest::Client::new();

        let url = format!(
            "http://127.0.0.1:3000/check_resv_with_date?year={}&month={}&day={}",
            date.year, date.month, date.day
        );

        // let url = "http://127.0.0.1:3000/check_resv?year=2024&month=5&day=25";
        let response = client.get(url).send().await?.text().await?;

        // Need to wrap Reservation in vector since the query returns a vec of reservation.
        let response: Vec<Reservation> = serde_json::from_str(&response)?;

        println!("{:?}", response);

        // assert_eq!(reqwest::StatusCode::OK, response.status());

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_resv() -> anyhow::Result<()> {
        let payload = serde_json::json!({
            "name": String::from("Jason"),
            "contact": String::from("+91123456789"),
            "seating": String::from("MainT7"),
            "specific_seating_requested": true,
            "advance": false,
            "advance_amount": Some(5000i32),
            "advance_method": Some("Card"),
            "confirmed": false,
            "reservation_date": NaiveDate::from_ymd_opt(2024i32, 05u32, 25u32).expect("failed to strcuture date while testing insert post request"),
            "reservation_time": time::Time::from_hms(14u8, 0u8,0u8).expect("failed to structure time for reservation."),
        });

        let client = reqwest::Client::new();

        let response = client
            .post("http://127.0.0.1:3000/create_resv")
            .json(&payload)
            .send()
            .await?;

        assert_eq!(reqwest::StatusCode::OK, response.status());

        Ok(())
    }
}
