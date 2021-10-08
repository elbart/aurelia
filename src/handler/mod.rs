use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};

pub mod authentication;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

use crate::database::model::Tag;
use crate::rbatis::crud::CRUD;

pub async fn get_tags(Extension(rb): Extension<Arc<Rbatis>>) -> impl IntoResponse {
    // (StatusCode::OK, Json(model::get_tags(state.0.clone())))
    let res: Vec<Tag> = rb.fetch_list().await.unwrap();
    Json(res)
}
