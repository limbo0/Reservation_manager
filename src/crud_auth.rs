use crate::helpers::salt_password;
use crate::models::{NewProperty, NewPropertyUser, Property, PropertyUsers, Roles};
use crate::SharedPooledConnection;
use axum::{
    extract::{Json, Path, Query, State},
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
};
use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use http::{header, HeaderMap, HeaderName, HeaderValue};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//TODO: Only devs can call this api.
#[axum_macros::debug_handler]
pub async fn show_all_properties(State(conn): State<SharedPooledConnection>) -> impl IntoResponse {
    use crate::schema::property::dsl::property;

    let results: Vec<Property> = property.load(&mut conn.try_get().unwrap()).unwrap();
    Json(results)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogIn {
    pub pid: Uuid,
    pub user_name: String,
    pub user_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    iss: String,
    sub: String,
    aud: String,
    exp: Option<String>,
    iat: DateTime<Utc>,
    jti: i32,
}

//TODO: finish this is in complete.
pub async fn auth(header: HeaderMap, next: Next) {}

pub async fn log_in(State(conn): State<SharedPooledConnection>, Json(payload): Json<LogIn>) {
    use crate::schema::{
        property::dsl::{property, property_id},
        propertyusers::dsl::user_name,
    };
    // check if property exists with the provided pid.
    let pp: Property = property
        .filter(property_id.eq(payload.pid))
        .select(Property::as_select())
        .get_result(&mut conn.try_get().unwrap())
        .unwrap();

    // check if user_name matches the provided user_name.
    let pu: PropertyUsers = PropertyUsers::belonging_to(&pp)
        .filter(user_name.eq(payload.user_name))
        .select(PropertyUsers::as_select())
        .get_result(&mut conn.try_get().unwrap())
        .unwrap();

    println!("enc-pwd: {:?}", pu.user_password);

    // check if provided password is correct.
    let jwtoken = match argon2::verify_encoded(&pu.user_password, payload.user_password.as_bytes())
        .unwrap()
    {
        true => {
            let mut nonce = 0;
            nonce += 1;
            let header = Header::default();
            let payload = Claims {
                iss: String::from("Server"),
                sub: pp.property_name.to_owned(),
                aud: pu.user_name.to_owned(),
                exp: None,
                iat: Utc::now(),
                jti: nonce,
            };
            //TODO: Secret has to be dynamic.
            let jw_token = encode(
                &header,
                &payload,
                &EncodingKey::from_secret("Secretforjwt".as_bytes()),
            )
            .expect("Failed to encode jwt token with header, claims and secret key");
            println!("Jwt: {:?}", jw_token);

            Redirect::to(&format!(
                "http://127.0.0.1:3000/reservations/{}",
                pp.property_id
            ))
        }
        false => panic!("wrong password"),
    };
    //NOTE: After the jwt token is created how is this sent back to the Client?
    // How will the server store the jwt token so it can verify while parsing the request header?

    //let mut headers = HeaderMap::new();
    //
    //headers.insert(
    //    HeaderName {
    //        inner: header::HOST,
    //    },
    //    "example.com".parse().unwrap(),
    //);
    //headers.insert(CONTENT_LENGTH, "123".parse().unwrap());
    //
    //headers
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
/// Create a new super user for the property with admin previlege.
//TODO: Should fail to create a property if the PropertyAdmin role isn't initialized in db.
#[axum_macros::debug_handler]
pub async fn create_property(
    State(conn): State<SharedPooledConnection>,
    Json(payload): Json<NewProperty>,
) -> impl IntoResponse {
    use crate::schema::{
        property::dsl::{property, property_id},
        propertyusers::dsl::{property_id as users_property_id, propertyusers},
    };

    println!("new property_id: {:?}", payload.property_id);

    // Creates a PropertyAdmin user account, while the property is initialized.
    let new_property_user = NewPropertyUser {
        user_name: String::from("PropertyAdmin"),
        user_password: String::from("Change_password_asap"),
        user_role: Roles::get_role(String::from("PropertyAdmin"), conn.clone())
            .expect("Failed to get role id from db."),
        property_id: payload.property_id,
    };

    diesel::insert_into(property)
        .values(NewProperty {
            property_id: payload.property_id,
            property_name: payload.property_name,
            property_password: salt_password(payload.property_password)
                .await
                .expect("Failed to salt password."),
            property_email: payload.property_email,
            property_phone: payload.property_phone,
        })
        .execute(&mut conn.try_get().unwrap())
        .expect("Error creating new property!");

    // Inserts newly created PropertyAdmin account and returns property_id.
    Json(
        diesel::insert_into(propertyusers)
            .values(new_property_user)
            .returning(users_property_id)
            .get_result::<Uuid>(&mut conn.try_get().unwrap())
            .expect("Failed to create new propertyuser"),
    )
}

//TODO: Only admin can call this api.
#[axum_macros::debug_handler]
pub async fn create_property_user(
    State(conn): State<SharedPooledConnection>,
    Json(payload): Json<NewPropertyUser>,
) -> impl IntoResponse {
    use crate::schema::propertyusers::dsl::{propertyusers, user_id};

    //TODO: Check user name and dont allow duplicate user names for same property.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login() -> anyhow::Result<()> {
        let payload = LogIn {
            pid: Uuid::parse_str("8ba0a933-6f7a-46d4-8378-a9ac2c457670")?,
            user_name: String::from("PropertyAdmin"),
            user_password: String::from("Change_password_asap"),
        };
        let client = reqwest::Client::new();
        let url = "http://127.0.0.1:3000/login";
        let response = client.post(url).json(&payload).send().await?;
        println!("{:?}", response);

        Ok(())
    }
}
