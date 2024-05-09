#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dt = chrono::offset::Local::now();
    println!(
        "current date: {:?}\ncurrent time: {:?}",
        dt.date_naive(),
        dt.time()
    );

    Ok(())
}
