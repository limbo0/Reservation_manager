use diesel_demo::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    io::{stdin, Read},
    str::FromStr,
};
use time::{Date, Month, Time};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    create_new_reservation(client).await?;
    // insert_resv(client).await?;

    Ok(())
}

async fn insert_resv(client: Client) -> anyhow::Result<()> {
    let response = client
        .post("http://127.0.0.1:3000/create_resv")
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Resv created successfully!");
    } else {
        println!("Failed to create resv: {}", response.status());
    }

    Ok(())
}
#[derive(Serialize, Deserialize)]
pub(crate) enum PaymentMethod {
    NotPaid,
    Cash,
    Card(Option<String>),
    Gpay(Option<String>),
}

/// Construct a new reservation
/// All the data fields will be passed via stdin atm.
async fn create_new_reservation(client: Client) -> anyhow::Result<()> {
    // _________________________________________________________________________________________
    println!("\nEnter reservation name\n");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();

    println!("\nEnter contact details\n",);
    let mut contact = String::new();
    stdin().read_to_string(&mut contact).unwrap();

    // _________________________________________________________________________________________
    println!("\nEnter table number\n");
    let mut seating = String::new();
    stdin().read_to_string(&mut seating).unwrap();

    println!("\nSpecific seating requested?\n");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("unable to read to buffer");
    let buf = buf.trim();
    let specific_seating_requested = match buf {
        "Y" | "y" => true,
        "N" | "n" => false,
        _ => false,
    };

    // _________________________________________________________________________________________
    println!("\nAdvance received? Enter Y or N\n");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("unable to read to buffer");

    let advance = match buf.trim() {
        "Y" | "y" => true,
        "N" | "n" => false,
        _ => false,
    };

    // Only predefined options should be available.
    let advance_method = match advance {
        true => {
            println!("\nHow was the advance paid? Enter: 0 -> NotPaid, 1 -> Cash, 2 -> Card, 3 -> Gpay\n");
            let mut buf_advance_method = String::new();
            stdin()
                .read_line(&mut buf_advance_method)
                .expect("Adcance payment method update failed.");

            match buf_advance_method
                .trim()
                .parse::<i32>()
                .expect("failed to parse PaymentMethod")
            {
                0 => PaymentMethod::NotPaid,
                1 => PaymentMethod::Cash,
                2 => {
                    let mut buf_card_slip_id = String::new();
                    println!("Enter card_slip_id of the payment.");
                    stdin()
                        .read_line(&mut buf_card_slip_id)
                        .expect("Failed to read card_slip_id to buffer.");

                    PaymentMethod::Card(Some(String::from(buf_card_slip_id.trim())))
                }
                3 => {
                    let mut buf_gpay_slip_id = String::new();
                    println!("Enter gpay_slip_id of the payment.");
                    stdin()
                        .read_line(&mut buf_gpay_slip_id)
                        .expect("Failed to read gpay_slip_id to buffer.");

                    PaymentMethod::Card(Some(String::from(buf_gpay_slip_id.trim())))
                }
                _ => PaymentMethod::NotPaid,
            }
        }
        false => PaymentMethod::NotPaid,
    };

    let advance_amount = match advance {
        true => {
            println!("\nEnter advance amount.\n");
            let mut buf_advance_amount = String::new();
            stdin()
                .read_line(&mut buf_advance_amount)
                .expect("Adcance amount failed");
            buf_advance_amount
                .trim()
                .parse::<i32>()
                .expect("failed to parse advance_amount")
        }
        false => 0i32,
    };

    // _________________________________________________________________________________________
    println!("\nConfirmed? Enter Y or N\n");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("unable to read to buffer");
    let confirmed = match buf.trim() {
        "Y" | "y" => true,
        "N" | "n" => false,
        _ => false,
    };

    // _________________________________________________________________________________________
    println!("\nEnter Year\n");
    let mut year = String::new();
    stdin().read_line(&mut year).expect("year error");

    println!("\nEnter Month\n");
    let mut month = String::new();
    stdin().read_line(&mut month).expect("month error");

    println!("\nEnter Date\n");
    let mut day = String::new();
    stdin().read_line(&mut day).expect("day error");

    let reservation_date = chrono::NaiveDate::from_ymd_opt(
        year.parse::<i32>().expect("Failed to parse year."),
        month.parse::<u32>().expect("Failed to parse month."),
        day.parse::<u32>().expect("Failed to parse day."),
    )
    .expect("Failed to structure reservation date.");

    // _________________________________________________________________________________________
    println!("\nEnter time: Hour\n");
    let mut hour = String::new();
    stdin().read_line(&mut hour).expect("Enter hour failed.");

    println!("\nEnter time: Minute\n");
    let mut minute = String::new();
    stdin()
        .read_line(&mut minute)
        .expect("Enter minute failed.");

    let reservation_time = Time::from_hms(
        hour.trim().parse::<u8>()?,
        minute.trim().parse::<u8>()?,
        u8::default(),
    )
    .expect("creating time for reservation failed.");

    // _________________________________________________________________________________________
    // all these params are use filled via stdin.
    let payload = serde_json::json!({
        "name": name.trim(),
        "contact": contact.trim(),
        "seating": seating.trim(),
        "specific_seating_requested": specific_seating_requested,
        "advance": advance,
        "advance_method": advance_method,
        "advance_amount": advance_amount,
        "confirmed": confirmed,
        "reservation_date": reservation_date,
        "reservation_time": reservation_time,
    });

    let response = client
        .post("http://127.0.0.1:3000/create_resv")
        .json(&payload)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Resv created successfully!");
    } else {
        println!("Failed to create resv: {}", response.status());
    }
    Ok(())
}
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
