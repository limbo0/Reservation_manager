use crate::helpers::salt_password;
use crate::models::{NewProperty, NewPropertyUser, Property, PropertyUsers, Roles};
use crate::SharedPooledConnection;
use axum::{
    extract::{Json, Path, Query, State},
    response::{Html, IntoResponse},
};
use diesel::prelude::*;
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    serialize::{Output, ToSql},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//TODO: Only devs can call this api.
#[axum_macros::debug_handler]
pub async fn show_all_properties(State(conn): State<SharedPooledConnection>) -> impl IntoResponse {
    use crate::schema::property::dsl::property;

    let results: Vec<Property> = property.load(&mut conn.try_get().unwrap()).unwrap();
    Json(results)
}

//TODO: Only property admin can call this api.
/// Returns all the users for the property.
#[axum_macros::debug_handler]
pub async fn show_property_users(
    Path(pid): Path<Uuid>,
    State(conn): State<SharedPooledConnection>,
) -> Json<Vec<PropertyUsers>> {
    use crate::schema::property::dsl::{property, property_id};

    let pp = property
        .filter(property_id.eq(pid))
        .select(Property::as_select())
        .get_result(&mut conn.try_get().unwrap())
        .unwrap();

    let pu = PropertyUsers::belonging_to(&pp)
        .select(PropertyUsers::as_select())
        .load(&mut conn.try_get().unwrap())
        .unwrap();

    Json(pu)
}

//NOTE: Sign in -> Creates a new property account.
// Create a new super user for the property with admin previlege.
// Send the user id and password to the provided email.
#[axum_macros::debug_handler]
pub async fn create_property(
    State(conn): State<SharedPooledConnection>,
    Json(payload): Json<NewProperty>,
) -> impl IntoResponse {
    use crate::schema::{
        property::dsl::{property, property_id},
        propertyusers::dsl::{property_id as users_property_id, propertyusers},
    };

    let pid = diesel::insert_into(property)
        .values(NewProperty {
            property_name: payload.property_name,
            property_password: salt_password(payload.property_password)
                .await
                .expect("Failed to salt password."),
            property_email: payload.property_email,
            property_phone: payload.property_phone,
        })
        .returning(property_id)
        .get_result::<Uuid>(&mut conn.try_get().unwrap())
        .expect("Error saving new user");
    println!("new property_id: {:?}", pid);

    //TODO: NewProperty should be the user input feilds on frontend.
    //TODO: Send username and password to provided email.
    let new_property_user = NewPropertyUser {
        user_name: String::from("PropertyAdmin"),
        user_password: String::from("Change_password_asap"),
        user_role: Roles::get_role(String::from("PropertyAdmin"), conn.clone())
            .expect("Failed to get role id from db."),
        property_id: pid,
    };

    Json(
        diesel::insert_into(propertyusers)
            .values(new_property_user)
            .returning(users_property_id)
            .get_result::<Uuid>(&mut conn.try_get().unwrap())
            .expect("Failed to create new propertyuser"),
    )
}

// CREATE TABLE [ IF NOT EXISTS ] property (
//   property_id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
//   property_name VARCHAR NOT NULL,
//   property_password VARCHAR NOT NULL,
//   property_email VARCHAR NOT NULL,
//   property_phone VARCHAR NOT NULL
// );
//

//TODO: Only admin can call this api.
#[axum_macros::debug_handler]
pub async fn create_property_user(
    State(conn): State<SharedPooledConnection>,
    Json(payload): Json<NewPropertyUser>,
) -> impl IntoResponse {
    use crate::schema::propertyusers::dsl::{propertyusers, user_id};

    let new_property_user = NewPropertyUser {
        user_name: payload.user_name,
        user_password: salt_password(payload.user_password)
            .await
            .expect("Failed to salt user password."),
        user_role: payload.user_role,
        property_id: payload.property_id,
    };

    Json(
        diesel::insert_into(propertyusers)
            .values(&new_property_user)
            .returning(user_id)
            .get_result::<i32>(&mut conn.try_get().unwrap())
            .expect("Error saving new user"),
    )
}
