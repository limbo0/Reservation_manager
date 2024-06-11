use chrono::Datelike;
use diesel_demo::{
    helpers::payment_mode_handler,
    models::{PaymentMethod, PaymentMode},
};
use reqwest::Client;
use std::io::{stdin, Read};
use time::Time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    create_new_reservation(client).await?;
    // insert_resv(client).await?;

    Ok(())
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

    let seating = match specific_seating_requested {
        true => {
            println!("\nEnter table which the guest has requested.\n");
            let mut seating = String::new();
            stdin().read_to_string(&mut seating).unwrap();
            seating
        }
        false => String::from("Table not specifically requested"),
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

    // Only predefined mode of payment should be possible.
    let advance_method = match advance {
        true => {
            println!("\nHow was the advance paid? 1 -> Cash, 2 -> Card, 3 -> Gpay\n");
            let mut buf_payment_mode = String::new();
            stdin()
                .read_line(&mut buf_payment_mode)
                .expect("Adcance payment method update failed.");

            match buf_payment_mode
                .trim()
                .parse::<i32>()
                .expect("failed to parse PaymentMethod")
            {
                1 => {
                    let (tx_id, receiver, received_date) = payment_mode_handler(1)
                        .await
                        .expect("Failed to construct information on payment method");
                    serde_json::value::to_value(PaymentMethod {
                        mode_of_payment: PaymentMode::Cash,
                        payment_transaction_id: tx_id,
                        payment_receiver: Some(receiver),
                        payment_received_date: Some(received_date),
                    })
                    .expect("failed to convert payment method to json")
                }
                2 => {
                    let (tx_id, receiver, received_date) = payment_mode_handler(2)
                        .await
                        .expect("Failed to construct information on payment method");
                    serde_json::value::to_value(PaymentMethod {
                        mode_of_payment: PaymentMode::Card,
                        payment_transaction_id: tx_id,
                        payment_receiver: Some(receiver),
                        payment_received_date: Some(received_date),
                    })
                    .expect("failed to convert payment method to json")
                }
                3 => {
                    let (tx_id, receiver, received_date) = payment_mode_handler(3)
                        .await
                        .expect("Failed to construct information on payment method");
                    serde_json::value::to_value(PaymentMethod {
                        mode_of_payment: PaymentMode::Gpay,
                        payment_transaction_id: tx_id,
                        payment_receiver: Some(receiver),
                        payment_received_date: Some(received_date),
                    })
                    .expect("failed to convert payment method to json")
                }
                _ => panic!("Unknown payment mode entered"),
            }
        }
        false => serde_json::value::to_value(PaymentMethod {
            mode_of_payment: PaymentMode::NotPaid,
            payment_transaction_id: None,
            payment_receiver: None,
            payment_received_date: None,
        })
        .expect("failed to convert payment method to json"),
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
    let current_date = chrono::offset::Local::now().date_naive();
    println!(
        "\nIs the reservation for today: {:?}?. Enter Y or N\n",
        current_date
    );
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("failed to read reservation for today to buf");

    let reservation_date = match buf.trim() {
        "y" | "Y" | "yes" | "Yes" => current_date,
        "n" | "N" | "no" | "No" => {
            println!("\nEnter Date\n");
            let mut day = String::new();
            stdin().read_line(&mut day).expect("day error");

            // _________________________________________________________________________________________
            let mut this_month = String::new();
            println!(
                "\nIs the reservation for this month: {:?}?. Enter Y or N\n",
                current_date.month()
            );
            stdin()
                .read_line(&mut this_month)
                .expect("failed to read this year to buf");

            let month = match this_month.trim() {
                "y" | "Y" | "yes" | "Yes" => current_date.month(),
                "n" | "N" | "no" | "No" => {
                    println!("\nEnter month\n");
                    let mut month = String::new();
                    stdin().read_line(&mut month).expect("month error");
                    month.parse::<u32>().expect("Failed to parse month.")
                }
                _ => panic!("Reservation month has to be answered (Y, Yes, N, No) only"),
            };

            // _________________________________________________________________________________________
            let mut this_year = String::new();
            println!(
                "\nIs the reservation for this year: {:?}?. Enter Y or N\n",
                current_date.year()
            );
            stdin()
                .read_line(&mut this_year)
                .expect("failed to read this year to buf");

            let year = match this_year.trim() {
                "y" | "Y" | "yes" | "Yes" => current_date.year(),
                "n" | "N" | "no" | "No" => {
                    println!("\nEnter Year\n");
                    let mut year = String::new();
                    stdin().read_line(&mut year).expect("year error");
                    year.parse::<i32>().expect("Failed to parse year.")
                }
                _ => panic!("Reservation year has to be answered (Y, Yes, N, No) only"),
            };

            chrono::NaiveDate::from_ymd_opt(
                year,
                month,
                day.parse::<u32>().expect("Failed to parse day."),
            )
            .expect("Failed to structure reservation date.")
        }
        _ => panic!("Reservation for today has to be answered (Y, Yes, N, No) only"),
    };

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
    println!("\nEnter property id.\n");
    let mut property_id = String::new();
    stdin()
        .read_line(&mut property_id)
        .expect("Enter pid to buffer failed.");

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
        "property_id": property_id.trim().parse::<uuid::Uuid>().expect("Failed to parse into uuid type"),
    });

    let response = client
        .post("http://127.0.0.1:9000/create_resv")
        .json(&payload)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Resv created successfully!");
        println!("returned data: {:?}", response.text().await?);
    } else {
        println!("Failed to create resv: {}", response.status());
    }
    Ok(())
}
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
