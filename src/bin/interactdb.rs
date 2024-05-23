use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use diesel_demo::{
    crud::{
        check_resv_with_date, check_resv_with_id, create_resv, delete_resv_with_id, read_resv_json,
        update_resv_date_with_id, update_resv_name_with_id,
    },
    crud_auth::{create_property, create_property_user, show_all_properties, show_property_users},
    crud_roles::create_new_role,
    get_connection_pool,
};

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();

    let app = Router::new()
        .route(
            "/",
            get(Html(String::from("Welcome to the reservation manager."))),
        )
        //NOTE: This should be the entrypoint for all clients. SIGN-UP
        .route("/create_property", post(create_property))
        .with_state(pool.clone())
        //NOTE: Create a new role in db.
        .route("/create_role", post(create_new_role))
        .with_state(pool.clone())
        //NOTE: Only available to the property admins.
        .route("/create_property_user", post(create_property_user))
        .with_state(pool.clone())
        //NOTE: Should display all the reservations for the current date.
        .route("/reservations/:pid", get(read_resv_json))
        //NOTE: Route should only be available to devs.
        .route("/property", get(show_all_properties))
        .with_state(pool.clone())
        //NOTE: Only available to the property admins.
        .route("/property_users/:pid", get(show_property_users))
        .with_state(pool.clone())
        // Create a new reservations and insert in db.
        .route("/create_resv", post(create_resv))
        .with_state(pool.clone())
        // Retrieves reservations of only the date specified on query.
        .with_state(pool.clone())
        .route("/check_resv_with_date", get(check_resv_with_date))
        // Retrieves reservations of only the id specified on path.
        .route("/check_resv_with_id/:resv_id", get(check_resv_with_id))
        .with_state(pool.clone())
        // Update the reservation name in db which matches the id passed in path.
        .route(
            "/update_resv_name_with_id/:resv_id/:new_name",
            post(update_resv_name_with_id),
        )
        .with_state(pool.clone())
        .route(
            "/update_resv_date_with_id/:resv_id/:new_date",
            post(update_resv_date_with_id),
        )
        .with_state(pool.clone())
        // Delete the reservation name from db which matches the id passed in path.
        .route("/delete_resv_with_id/:resv_id", post(delete_resv_with_id))
        .with_state(pool.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
