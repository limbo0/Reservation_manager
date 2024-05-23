use crate::models::Roles;
use crate::SharedPooledConnection;
use axum::{
    extract::{Json, Path, Query, State},
    response::{Html, IntoResponse},
};
use diesel::prelude::*;

//TODO: secure this function call.
#[axum_macros::debug_handler]
pub async fn create_new_role(
    State(conn): State<SharedPooledConnection>,
    Json(payload): Json<Roles>,
) -> impl IntoResponse {
    use crate::schema::roles::dsl::{role_id, roles};

    let new_role = Roles {
        role_id: payload.role_id,
        role_name: payload.role_name,
    };

    Json(
        diesel::insert_into(roles)
            .values(new_role)
            .returning(role_id)
            .get_result::<i32>(&mut conn.try_get().unwrap())
            .expect("Error saving new role in db"),
    )
}
