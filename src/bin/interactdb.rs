use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use diesel_demo::{
    create_resv, delete_resv_with_id, read_resv_json, read_resv_with_date, read_resv_with_id,
    update_resv_name_with_id,
};

#[tokio::main]
async fn main() {
    // let connection = &mut establish_connection();
    let app = Router::new()
        .route("/", get(read_resv_json))
        .route("/create_resv", post(create_resv))
        .route("/check_resv_with_date", get(read_resv_with_date))
        .route("/check_resv_with_id/:resv_id", get(read_resv_with_id))
        .route(
            "/update_resv_name_with_id/:resv_id/:new_name",
            post(update_resv_name_with_id),
        )
        .route("/delete_resv_with_id/:resv_id", post(delete_resv_with_id));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
