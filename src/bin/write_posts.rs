use diesel_demo::*;
use std::io::{stdin, Read};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut contact = String::new();
    let mut seating = String::new();

    // println!("\nEnter unique id");
    // let mut buf = String::new();
    // stdin().read_line(&mut buf).unwrap();
    // let id: i32 = buf.trim().parse::<i32>().unwrap();

    println!("\nEnter reservation name\n");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!("\nEnter contact details\n",);
    stdin().read_to_string(&mut contact).unwrap();

    println!("\nEnter table number\n");
    stdin().read_to_string(&mut seating).unwrap();

    println!("\nadvance received? Enter Y or N\n");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("unable to read to buffer");

    let buf = buf.trim();
    let advance = match buf {
        "Y" | "y" => true,
        "N" | "n" => false,
        _ => false,
    };

    println!("\nconfirmed? Enter Y or N\n");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("unable to read to buffer");

    let buf = buf.trim();
    let confirmed = match buf {
        "Y" | "y" => true,
        "N" | "n" => false,
        _ => false,
    };

    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "name": name,
        "contact": contact,
        "seating": seating,
        "advance": advance,
        "confirmed": confirmed,
    });

    let response = client
        .post("http://127.0.0.1:3000/create_resv")
        .json(&payload)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("User created successfully!");
    } else {
        println!("Failed to create user: {}", response.status());
    }

    Ok(())
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
