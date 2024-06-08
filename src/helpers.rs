use crate::models::{NewResv, PaymentMethod, PaymentMode};
use argon2::{self, Config};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::PgConnection;
use rand::RngCore;
use std::io::stdin;

/// Password hasher.
pub async fn salt_password(secret: String) -> anyhow::Result<String> {
    let mut salt = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut salt);
    // println!("salt: {:?}", salt);

    let config = Config::default();
    let hash_p = argon2::hash_encoded(secret.as_bytes(), &salt, &config).unwrap();
    Ok(hash_p)
}

/// Helper function to construct data structure for PaymentMethod for reservation input in db.
/// All the payment modes are handled similarly besides payment mode cash.
pub async fn payment_mode_handler(
    payment_mode: i32,
) -> anyhow::Result<(Option<String>, String, NaiveDate)> {
    let payment_method_data = match payment_mode {
        // Handler if the payment mode is cash.
        // Payment_transaction_id is returned None.
        1 => {
            let mut receiver = String::new();
            println!("\nEnter name of the payment receiver.\n");
            stdin()
                .read_line(&mut receiver)
                .expect("Failed to read payment receiver to the buffer");

            let mut received_today = String::new();
            println!("\nAdvance received_today? Enter Y or N\n");
            stdin()
                .read_line(&mut received_today)
                .expect("Failed to read reveived_today to the buffer");

            let payment_received_date = match received_today.trim() {
                // If true then we construct todays date.
                "Y" | "y" | "yes" | "Yes" => chrono::offset::Local::now().date_naive(),
                // If false we ask for user input.
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
        // All payment modes other than cash is handled the same.
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
