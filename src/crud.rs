use crate::models::{NewResv, Reservation};
use crate::SharedPooledConnection;
use axum::{
    extract::{Json, Path, Query, State},
    response::{Html, IntoResponse},
};
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//NOTE: CREATE
//Returns reservation id after sucessfully updating db.
#[axum_macros::debug_handler]
pub async fn create_resv(
    State(conn): State<SharedPooledConnection>,
    Json(payload): Json<NewResv>,
) -> Json<Vec<i32>> {
    //TODO: Send the necessary data to the provided contacts.
    use crate::schema::reservation::dsl::{id, reservation};
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
        property_id: payload.property_id,
    };

    Json(
        diesel::insert_into(reservation)
            .values(&new_resv)
            .returning(id)
            .get_results::<i32>(&mut conn.try_get().unwrap())
            .expect("Error saving new resv"),
    )
}

//NOTE: READ
//NOTE: Return with ascending order in time of upcoming reservations.
//TODO: Match the property id and only return reservation of that id.
/// Check the current date and return all reservations for that date.
#[axum_macros::debug_handler]
pub async fn read_resv_json(
    State(conn): State<SharedPooledConnection>,
    Path(pid): Path<Uuid>,
) -> Json<Vec<Reservation>> {
    use crate::schema::reservation::dsl::{property_id, reservation, reservation_date};

    //TODO: Set it to Utc instead of local.
    // check the current date and return all reservations for that date.
    let results = reservation
        .filter(property_id.eq(pid))
        .filter(reservation_date.eq(chrono::offset::Local::now().date_naive()))
        .limit(5)
        .select(Reservation::as_select())
        .load(&mut conn.try_get().unwrap())
        .expect("Error loading resv");

    Json(results)
}

/// Returns reservation with matching date.
#[axum_macros::debug_handler]
pub async fn check_resv_with_date(
    Query(date): Query<MyDate>,
    State(conn): State<SharedPooledConnection>,
) -> Json<Vec<Reservation>> {
    use crate::schema::reservation::dsl::{reservation, reservation_date};

    let resv_date = NaiveDate::from_ymd_opt(date.year, date.month, date.day)
        .expect("failed creating date for query");

    let results = reservation
        .filter(reservation_date.eq(resv_date))
        .select(Reservation::as_select())
        .load(&mut conn.try_get().unwrap())
        .expect("Error loading resv");

    Json(results)
}

/// Returns reservation with matching id.
//TODO: Handle while reservation for Id is not found.
pub async fn check_resv_with_id(
    Path(resv_id): Path<i32>,
    State(conn): State<SharedPooledConnection>,
) -> impl IntoResponse {
    use crate::schema::reservation::dsl::{id, reservation};
    let results = reservation
        .filter(id.eq(resv_id))
        .select(Reservation::as_select())
        .load(&mut conn.try_get().unwrap())
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
pub async fn update_resv_name_with_id(
    Path((resv_id, new_name)): Path<(i32, String)>,
    State(conn): State<SharedPooledConnection>,
) {
    use crate::schema::reservation::dsl::{id, name, reservation};

    diesel::update(reservation)
        .filter(id.eq(resv_id))
        .set(name.eq(new_name))
        .execute(&mut conn.try_get().unwrap())
        .expect(&format!(
            "Failed to update name for reservation Id: {:?}",
            resv_id
        ));
}

/// Checks if the reservation with the id exists, if true then update date.
pub async fn update_resv_date_with_id(
    Path((resv_id, new_date)): Path<(i32, NaiveDate)>,
    State(conn): State<SharedPooledConnection>,
) {
    use crate::schema::reservation::dsl::{id, reservation, reservation_date};

    diesel::update(reservation)
        .filter(id.eq(resv_id))
        .set(reservation_date.eq(new_date))
        .execute(&mut conn.try_get().unwrap())
        .expect(&format!(
            "Failed to update date for reservation Id: {:?}",
            resv_id
        ));
}

///NOTE: DELETE
pub async fn delete_resv_with_id(
    Path(resv_id): Path<i32>,
    State(conn): State<SharedPooledConnection>,
) {
    use crate::schema::reservation::dsl::{id, reservation};

    //TODO: Error handle if the given resv_id don't exists.
    diesel::delete(reservation)
        .filter(id.eq(resv_id))
        .execute(&mut conn.try_get().unwrap())
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
            day: 16u32,
        };
        let client = reqwest::Client::new();
        let url = format!(
            "http://127.0.0.1:3000/check_resv_with_date?year={}&month={}&day={}",
            date.year, date.month, date.day
        );
        let response = client.get(url).send().await?.text().await?;
        // Need to wrap Reservation in vector since the query returns a vec of reservation.
        let response_data: Vec<Reservation> = serde_json::from_str(&response)?;
        println!("{:?}", response_data);

        // assert_eq!(reqwest::StatusCode::OK, response.status());

        Ok(())
    }

    #[tokio::test]
    ///NOTE: This test needs to be updated before using again.
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
    e   });

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
