use crate::models::user::{NewUser, User};
use crate::schema;
use axum::{
    Json as AxumJson,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

pub async fn create_user(
    State(pool): State<Pool<ConnectionManager<diesel::PgConnection>>>,
    Json(body): Json<CreateUser>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection failed",
            )
                .into_response();
        }
    };

    let new_user = NewUser {
        username: &body.username,
        email: &body.email,
        avatar_url: None,
    };

    match diesel::insert_into(schema::users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
    {
        Ok(user) => (StatusCode::CREATED, AxumJson(user)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response(),
    }
}
