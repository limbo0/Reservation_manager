use reqwest::Client;
use std::io::stdin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    // update_resv_name_with_id(&client).await?;
    update_resv_date_with_id(&client).await?;

    Ok(())
}
/// Makes a post request without a body.
async fn update_resv_date_with_id(client: &Client) -> anyhow::Result<()> {
    println!("\nEnter reservation Id.\n");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("failed to read id to the buffer");
    let id = buf
        .trim()
        .parse::<i32>()
        .expect("failed to parse id to type i32");

    println!("Enter new reservation date.");
    println!("\nEnter Year\n");
    let mut year = String::new();
    stdin().read_line(&mut year).expect("year error");

    println!("\nEnter Month\n");
    let mut month = String::new();
    stdin().read_line(&mut month).expect("month error");

    println!("\nEnter Date\n");
    let mut day = String::new();
    stdin().read_line(&mut day).expect("day error");

    let new_reservation_date = chrono::NaiveDate::from_ymd_opt(
        year.parse::<i32>().expect("Failed to parse year."),
        month.parse::<u32>().expect("Failed to parse month."),
        day.parse::<u32>().expect("Failed to parse day."),
    )
    .expect("Failed to structure reservation date.");

    let response = client
        .post(format!(
            "http://127.0.0.1:3000/update_resv_date_with_id/{:?}/{:?}",
            id, new_reservation_date
        ))
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Resv updated successfully!");
    } else {
        println!("Failed to update resv: {}", response.status());
    }

    Ok(())
}

/// Makes a post request without a body.
async fn update_resv_name_with_id(client: &Client) -> anyhow::Result<()> {
    println!("\nEnter reservation Id.\n");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("failed to read id to the buffer");
    let id = buf
        .trim()
        .parse::<i32>()
        .expect("failed to parse id to type i32");

    println!("\nEnter new reservation name.\n");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("failed to read id to the buffer");
    let new_name = String::from(buf.trim());

    let response = client
        .post(format!(
            "http://127.0.0.1:3000/update_resv_name_with_id/{:?}/{:?}",
            id, new_name
        ))
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Resv updated successfully!");
    } else {
        println!("Failed to update resv: {}", response.status());
    }

    Ok(())
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
