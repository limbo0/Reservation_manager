use crate::models::{NewResv, PaymentMethod, PaymentMode};
use argon2::{self, Config};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::PgConnection;
use rand::RngCore;
use std::io::stdin;

pub async fn salt_password(secret: String) -> anyhow::Result<String> {
    let mut salt = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut salt);
    // println!("salt: {:?}", salt);

    let config = Config::default();
    let hash_p = argon2::hash_encoded(secret.as_bytes(), &salt, &config).unwrap();
    Ok(hash_p)
}

/// Helper function to construct data structure for PaymentMethod for reservation input in db.
/// Input handler: if 1 Aka PaymentMode::Cash is passed then we dont care about tx_id.
pub async fn let_user_input(
    payment_mode: i32,
) -> anyhow::Result<(Option<String>, String, NaiveDate)> {
    let payment_method_data = match payment_mode {
        1 => {
            let mut receiver = String::new();
            println!("\nEnter name of the payment receiver.\n");
            stdin()
                .read_line(&mut receiver)
                .expect("Failed to read tx_id to  the buffer");

            let mut received_today = String::new();
            println!("\nAdvance received_today? Enter Y or N\n");
            stdin()
                .read_line(&mut received_today)
                .expect("Failed to read tx_id to  the buffer");

            let payment_received_date = match received_today.trim() {
                "Y" | "y" | "yes" | "Yes" => chrono::offset::Local::now().date_naive(),
                "N" | "n" | "no" | "No" => {
                    println!("\nEnter payment received year\n");
                    let mut year = String::new();
                    stdin().read_line(&mut year).expect("year error");

                    println!("\nEnter payment received month\n");
                    let mut month = String::new();
                    stdin().read_line(&mut month).expect("month error");

                    println!("\nEnter payment received date\n");
                    let mut day = String::new();
                    stdin().read_line(&mut day).expect("day error");

                    chrono::NaiveDate::from_ymd_opt(
                        year.parse::<i32>().expect("Failed to parse year."),
                        month.parse::<u32>().expect("Failed to parse month."),
                        day.parse::<u32>().expect("Failed to parse day."),
                    )
                    .expect("Failed to structure reservation date.")
                }
                _ => panic!("Payment received today should be answered with Y or N"),
            };

            (None, receiver, payment_received_date)
        }
        _ => {
            let mut tx_id = String::new();
            println!("\nEnter tx_id of payment receipt.\n");
            stdin()
                .read_line(&mut tx_id)
                .expect("Failed to read tx_id to  the buffer");

            let mut receiver = String::new();
            println!("\nEnter name of the payment receiver.\n");
            stdin()
                .read_line(&mut receiver)
                .expect("Failed to read tx_id to  the buffer");

            let mut received_today = String::new();
            println!("\nAdvance received_today? Enter Y or N\n");
            stdin()
                .read_line(&mut received_today)
                .expect("Failed to read tx_id to  the buffer");

            let payment_received_date = match received_today.trim() {
                "Y" | "y" | "yes" | "Yes" => chrono::offset::Local::now().date_naive(),
                "N" | "n" | "no" | "No" => {
                    println!("\nEnter payment received year\n");
                    let mut year = String::new();
                    stdin().read_line(&mut year).expect("year error");

                    println!("\nEnter payment received month\n");
                    let mut month = String::new();
                    stdin().read_line(&mut month).expect("month error");

                    println!("\nEnter payment received date\n");
                    let mut day = String::new();
                    stdin().read_line(&mut day).expect("day error");

                    chrono::NaiveDate::from_ymd_opt(
                        year.parse::<i32>().expect("Failed to parse year."),
                        month.parse::<u32>().expect("Failed to parse month."),
                        day.parse::<u32>().expect("Failed to parse day."),
                    )
                    .expect("Failed to structure reservation date.")
                }
                _ => panic!("Payment received today should be answered with Y or N"),
            };

            (Some(tx_id), receiver, payment_received_date)
        }
    };

    Ok(payment_method_data)
}

// Test purpose insertion.
pub async fn insert_resv(conn: &mut PgConnection) {
    let data = NewResv {
        name: String::from("Boa Hancock"),
        contact: String::from("+91123456789"),
        seating: String::from("MainT7"),
        specific_seating_requested: true,
        advance: true,
        advance_method: serde_json::value::to_value(PaymentMethod {
            mode_of_payment: PaymentMode::Card,
            payment_transaction_id: Some(String::from("0x1dac45")),
            payment_receiver: Some(String::from("Limboo")),
            payment_received_date: Some(
                NaiveDate::from_ymd_opt(2024i32, 05u32, 14u32)
                    .expect("failed to strcuture date while testing insert post request"),
            ),
        })
        .expect("failed to convert payment method to json"),
        advance_amount: Some(5000i32),
        confirmed: true,
        reservation_date: NaiveDate::from_ymd_opt(2024i32, 05u32, 14u32)
            .expect("failed to strcuture date while testing insert post request"),
        reservation_time: time::Time::from_hms(14u8, 0u8, 0u8)
            .expect("failed to structure time for reservation."),
        property_id: uuid::Uuid::new_v4(),
    };

    use crate::schema::reservation::dsl::reservation;

    diesel::insert_into(reservation)
        .values(&data)
        .execute(conn)
        .expect("Error saving new resv");
}
