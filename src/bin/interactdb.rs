use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use diesel_demo::{
    create_resv, delete_resv_with_id, insert_resv, read_resv_json, read_resv_with_date,
    read_resv_with_id, update_resv_date_with_id, update_resv_name_with_id,
};

#[tokio::main]
async fn main() {
    // _________________________________________________________________________________________
    let app = Router::new()
        // Should display all the reservations for the current date.
        .route("/", get(read_resv_json))
        // Create a new reservations and insert in db.
        .route("/create_resv", post(create_resv))
        // Retrieves reservations of only the date specified on query.
        .route("/check_resv_with_date", get(read_resv_with_date))
        // Retrieves reservations of only the id specified on path.
        .route("/check_resv_with_id/:resv_id", get(read_resv_with_id))
        // Update the reservation name in db which matches the id passed in path.
        .route(
            "/update_resv_name_with_id/:resv_id/:new_name",
            post(update_resv_name_with_id),
        )
        .route(
            "/update_resv_date_with_id/:resv_id/:new_date",
            post(update_resv_date_with_id),
        )
        // Delete the reservation name from db which matches the id passed in path.
        .route("/delete_resv_with_id/:resv_id", post(delete_resv_with_id));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
