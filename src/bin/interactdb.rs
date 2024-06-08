use axum::{
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use diesel_demo::{
    crud::{
        check_resv_with_date, check_resv_with_id, create_resv, delete_resv_with_id, read_resv_json,
        update_resv_date_with_id, update_resv_name_with_id,
    },
    crud_auth::{
        create_property, create_property_user, log_in, show_all_properties, show_property_users,
    },
    crud_roles::create_new_role,
    get_connection_pool,
};
use leptos::{component, create_signal, mount_to_body, view, IntoView};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();

    let dev_routes = Router::new()
        .route("/create_role", post(create_new_role))
        .with_state(pool.clone())
        .route("/property", get(show_all_properties))
        .with_state(pool.clone())
        .route("/property_users/:pid", get(show_property_users))
        .with_state(pool.clone());

    let property_admin_routes = Router::new()
        .route("/create_property", post(create_property))
        .with_state(pool.clone())
        .route("/create_property_user", post(create_property_user))
        .with_state(pool.clone());

    let property_user_routes = Router::new()
        .route("/login", post(log_in))
        .with_state(pool.clone())
        .route("/create_resv", post(create_resv))
        .with_state(pool.clone())
        .route("/reservations/:pid", get(read_resv_json))
        .with_state(pool.clone())
        .route("/check_resv_with_date", get(check_resv_with_date))
        .with_state(pool.clone())
        .route("/check_resv_with_id/:resv_id", get(check_resv_with_id))
        .with_state(pool.clone())
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
        .route("/delete_resv_with_id/:resv_id", post(delete_resv_with_id))
        .with_state(pool.clone());

    let app = Router::new()
        .route("/", get(Html("Welcome to reservations manager.")))
        .merge(dev_routes)
        .merge(property_admin_routes)
        .merge(property_user_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
