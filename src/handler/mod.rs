use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use sea_orm::{entity::*, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::{
    database::entity::recipe as Recipe, database::entity::tag as Tag,
    middleware::authentication::JwtClaims,
};

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

pub async fn get_tags(Extension(db): Extension<Arc<DatabaseConnection>>) -> impl IntoResponse {
    let tags = Tag::Entity::find().all(&db).await.unwrap();

    (StatusCode::OK, Json(tags))
}

pub async fn get_recipes(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Extension(claims): Extension<Option<JwtClaims>>,
) -> impl IntoResponse {
    assert!(claims.is_some());
    let recipes = Recipe::Entity::find().all(&db).await.unwrap();

    (StatusCode::OK, Json(recipes))
}
