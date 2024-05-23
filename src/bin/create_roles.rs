use diesel_demo::models::Roles;
use reqwest::Client;
use std::io::stdin;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    create_role(client).await?;
    Ok(())
}

async fn create_role(client: Client) -> anyhow::Result<()> {
    println!("\nEnter role_id\n");
    let mut role_id = String::new();
    stdin().read_line(&mut role_id).unwrap();

    println!("\nEnter role_name\n");
    let mut role_name = String::new();
    stdin().read_line(&mut role_name).unwrap();

    let payload = serde_json::json!({
        "role_id": role_id.trim().parse::<i32>().unwrap(),
        "role_name": role_name.trim(),
    });

    let response = client
        .post("http://127.0.0.1:3000/create_role")
        .json(&payload)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Role created successfully!");
        println!("returned data: {:?}", response.text().await?);
    } else {
        println!("Failed to create role: {}", response.status());
    }
    Ok(())
}
