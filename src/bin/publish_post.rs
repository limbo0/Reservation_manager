use self::models::Reservation;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main() {
    use self::schema::resv::dsl::{confirmed, resv};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = diesel::update(resv.find(id))
        .set(confirmed.eq(true))
        .returning(Reservation::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Published post {}", post.name);
}
