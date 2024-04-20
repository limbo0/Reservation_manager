use self::models::*;
use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use diesel_demo::*;

#[tokio::main]
async fn main() {
    // let connection = &mut establish_connection();
    let app = Router::new()
        .route("/", get(show_resv))
        .route("/create_resv", post(insert_resv));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn show_resv() -> Json<Vec<Reservation>> {
    use self::schema::resv::dsl::*;
    let results = resv
        .filter(confirmed.eq(true))
        .limit(5)
        .select(Reservation::as_select())
        .load(&mut diesel_demo::establish_connection())
        .expect("Error loading posts");

    // let data: Vec<Reservation> = results;

    Json(results)
}
