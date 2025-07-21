use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

pub fn create_user(
    State(conn): State<diesel::PgConnection>,
    Json(body): Json<CreateUser>,
) -> impl IntoResponse {
    use crate::schema::users;

    let new_user = NewUser {
        username: &body.username,
        email: &body.email,
        avatar_url: None, // Assuming avatar_url is optional and not provided in this case
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(crate::models::user::User::as_returning())
        .get_result(&conn)
}
