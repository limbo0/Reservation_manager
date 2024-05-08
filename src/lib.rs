use self::models::{NewResv, Reservation};
use crate::schema::reservation;
use axum::{
    extract::{Json, Path, Query},
    response::{Html, IntoResponse},
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, time::SystemTime};
use time::{Date, Month};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// On backends that support it, you can call `get_result` instead of `execute`
// can explicitly return an expression by using the `returning` method before
// to have `RETURNING *` automatically appended to the query. Alternatively, you
// getting the result.

//NOTE: CREATE
pub async fn create_resv(Json(payload): Json<NewResv>) {
    // build the data structure
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
    diesel::insert_into(reservation::table)
        .values(&new_resv)
        .returning(Reservation::as_returning())
        .execute(&mut establish_connection())
        .expect("Error saving new resv");
}

//NOTE: READ
//NOTE: Return with ascending order of upcoming reservations.
// Shows all the confirmed reservation while the page is loaded.
// Returning in Html, to implement Htmx (test and see)

#[axum_macros::debug_handler]
pub async fn read_resv_json() -> impl IntoResponse {
    use self::schema::reservation::dsl::*;
    let results = reservation
        .filter(
            reservation_date.gt(Date::from_calendar_date(2024i32, Month::January, 1u8).unwrap()),
        )
        .limit(5)
        .select(Reservation::as_select())
        .load(&mut establish_connection())
        .expect("Error loading resv");

    Json(results)
}

/// Returns reservation with matching date.
#[axum_macros::debug_handler]
pub async fn check_resv_with_date(Query(date): Query<MyDate>) -> Json<Vec<Reservation>> {
    use self::schema::reservation::dsl::*;

    let resv_date = Date::from_calendar_date(date.year, date.month, date.day)
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
pub async fn check_resv_with_id(Path(resv_id): Path<i32>) -> impl IntoResponse {
    use self::schema::reservation::dsl::*;
    let results = reservation
        .filter(id.eq(resv_id))
        .select(Reservation::as_select())
        .load(&mut establish_connection())
        .expect("Couldn't find reservation with provided id.");

    Json(results)
    // Html(results[0].to_string())
}

//NOTE: UPDATE
/// Checks if the reservation with the id exists, if true then update.
pub async fn update_resv_name_with_id(Path((resv_id, new_name)): Path<(i32, String)>) {
    use self::schema::reservation::dsl::*;

    diesel::update(reservation)
        .filter(id.eq(resv_id))
        .set(name.eq(new_name))
        .execute(&mut establish_connection())
        .expect("Failed to update new_name");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyDate {
    year: i32,
    month: Month,
    day: u8,
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
            month: Month::March,
            day: 25u8,
        };

        let client = reqwest::Client::new();

        //NOTE: while making a get request, pass the month as number not as type(Month)
        let url = format!(
            "http://127.0.0.1:3000/check_resv?year={}&month={}&day={}",
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
            "reservation_date": Date::from_calendar_date(2024i32, Month::May, 25u8).expect("failed to strcuture date while testing insert post request"),
            "reservation_time": time::Time::from_hms(14u8, 0u8,0u8).expect("failed to structure time for reservation."),
        });

        let client = reqwest::Client::new();

        let response = client
            .post("http://127.0.0.1:3000/insert_resv")
            .json(&payload)
            .send()
            .await?;

        assert_eq!(reqwest::StatusCode::OK, response.status());

        Ok(())
    }
}
