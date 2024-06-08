use axum::http::Request;
use reqwest::Client;
use std::io::stdin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    log_in(client).await?;
    Ok(())
}

async fn log_in(client: Client) -> anyhow::Result<()> {
    println!("\nEnter pid\n");
    let mut pid = String::new();
    stdin().read_line(&mut pid).unwrap();

    println!("\nEnter username\n");
    let mut user_name = String::new();
    stdin().read_line(&mut user_name).unwrap();

    println!("\nEnter user password\n");
    let mut user_password = String::new();
    stdin().read_line(&mut user_password).unwrap();

    // let payload = diesel_demo::crud_auth::LogIn {
    //     pid: pid.trim().parse::<uuid::Uuid>().unwrap(),
    //     user_name,
    //     user_password,
    // };

    let payload = serde_json::json!({
        "pid": pid.trim().parse::<uuid::Uuid>().unwrap(),
        "user_name": user_name.trim(),
        "user_password": user_password.trim(),
    });

    let response = client
        .post("http://127.0.0.1:3000/login")
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
