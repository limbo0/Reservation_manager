use core::panic;
use reqwest::Client;
use std::io::stdin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let mut buf = String::new();
    println!("\nEnter: p to update property or pu to update property_users\n");
    stdin().read_line(&mut buf).unwrap();
    match buf.trim() {
        "p" | "P" => create_new_property(client.clone()).await?,
        "pu" | "Pu" => create_new_property_user(client.clone()).await?,
        _ => panic!("neither p or pu selected"),
    }

    Ok(())
}

async fn create_new_property_user(client: Client) -> anyhow::Result<()> {
    println!("\nPlease enter user name\n");
    let mut user_name = String::new();
    stdin()
        .read_line(&mut user_name)
        .expect("failed to read data to buffer");

    println!("\nPlease enter user password\n");
    let mut user_passwd = String::new();
    stdin()
        .read_line(&mut user_passwd)
        .expect("failed to read data to buffer");

    println!("\nPlease enter user role\n");
    let mut user_role = String::new();
    stdin()
        .read_line(&mut user_role)
        .expect("failed to read data to buffer");

    println!("\nPlease enter property id\n");
    let mut property = String::new();
    stdin()
        .read_line(&mut property)
        .expect("failed to read data to buffer");

    let payload = serde_json::json!({
        "user_name": user_name.trim(),
        "user_password": user_passwd.trim(),
        "user_role": user_role.trim().parse::<i32>().expect("failed to parse user role"),
        "property_id": property.trim().parse::<uuid::Uuid>().expect("failed to parse property id "),

    });

    let response = client
        .post("http://127.0.0.1:3000/create_property_user")
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

//NOTE: sign up
async fn create_new_property(client: Client) -> anyhow::Result<()> {
    println!("\nPlease enter property name\n");
    let mut property_name = String::new();
    stdin()
        .read_line(&mut property_name)
        .expect("failed to read data to buffer");

    println!("\nPlease enter property password\n");
    let mut property_passwd = String::new();
    stdin()
        .read_line(&mut property_passwd)
        .expect("failed to read data to buffer");

    println!("\nPlease enter property email\n");
    let mut property_email = String::new();
    stdin()
        .read_line(&mut property_email)
        .expect("failed to read data to buffer");

    println!("\nPlease enter property phone number\n");
    let mut property_phone = String::new();
    stdin()
        .read_line(&mut property_phone)
        .expect("failed to read data to buffer");

    let payload = serde_json::json!({
        "property_id": uuid::Uuid::new_v4(),
        "property_name": property_name.trim(),
        "property_password": property_email.trim(),
        "property_email": property_email.trim(),
        "property_phone": property_phone.trim(),

    });

    let response = client
        .post("http://127.0.0.1:3000/create_property")
        .json(&payload)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("New property created successfully!");
        println!("returned data: {:?}", response.text().await?);
    } else {
        println!("Failed to create resv: {}", response.status());
    }

    Ok(())
}
