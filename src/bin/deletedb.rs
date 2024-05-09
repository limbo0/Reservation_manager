use reqwest::Client;
use std::io::stdin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    delete_db_with_id(client).await?;

    Ok(())
}

async fn delete_db_with_id(client: Client) -> anyhow::Result<()> {
    println!("Enter reservation id which is to be deleted.\n");
    let mut resv_id = String::new();
    stdin()
        .read_line(&mut resv_id)
        .expect("Failed to read input to buffer.");

    let response = client
        .post(format!(
            "http://127.0.0.1:3000/delete_resv_with_id/{:?}",
            resv_id
                .trim()
                .parse::<i32>()
                .expect("Failed to parse reservation id.")
        ))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Resv deleted sucessfullt.");
    } else {
        println!(
            "Failed to delete reservation with status code {:?}",
            response.status()
        );
    }

    Ok(())
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
