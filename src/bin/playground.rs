use argon2::Config;
use base64ct::{Base64, Encoding};
use hex_literal::hex;
use lettre::message::{header::ContentType, Message};
use rand::RngCore;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let dt = chrono::offset::Local::now();
    // println!(
    //     "current date: {:?}\ncurrent time: {:?}",
    //     dt.date_naive(),
    //     dt.time()
    // );

    let mut salt = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut salt);
    println!("salt: {:?}", salt);

    let config = Config::default();
    let hash_p = argon2::hash_encoded("Eva".as_bytes(), &salt, &config).unwrap();
    println!("hashed: {:?}", hash_p);

    let checking = argon2::verify_encoded(&hash_p, "eva".as_bytes()).unwrap();
    match checking {
        true => println!("{:?}", String::from("matched")),
        _ => println!("{:?}", String::from("Not matched")),
    };

    // let mut hasher = sha2::Sha256::new();
    // hasher.update("khewa");
    // let results = hasher.finalize();
    //
    // println!("first: {:?}", results);
    //
    // let encoded = Base64::encode_string(&results);
    // println!("hashedP: {:?}", encoded);
    //
    // let mut buf: Vec<u8> = vec![0; 1000];
    // let decoded = Base64::decode(encoded, &mut buf).unwrap();
    //
    // println!("hashedP: {:?}", decoded);

    Ok(())
}
