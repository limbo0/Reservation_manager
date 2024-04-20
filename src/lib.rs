use axum::{extract::Json, Extension};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewResv, Reservation};

pub async fn insert_resv(Json(payload): Json<NewResv>) {
    use crate::schema::resv;

    let new_resv = NewResv {
        name: payload.name,
        contact: payload.contact,
        seating: payload.seating,
        advance: payload.advance,
        confirmed: payload.confirmed,
    };

    diesel::insert_into(resv::table)
        .values(&new_resv)
        .returning(Reservation::as_returning())
        .execute(&mut establish_connection())
        .expect("Error saving new resv");
}
