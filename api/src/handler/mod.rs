use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{
        entity::{ingredient, recipe, tag},
        DbPool,
    },
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

pub async fn get_tags(
    Extension(claims): Extension<Option<JwtClaims>>,
    Extension(db): Extension<DbPool>,
) -> impl IntoResponse {
    assert!(claims.is_some());
    (StatusCode::OK, Json(tag::get_tags(None, db).await))
}

pub async fn get_tag(
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Option<JwtClaims>>,
    Extension(db): Extension<DbPool>,
) -> impl IntoResponse {
    assert!(claims.is_some());
    (StatusCode::OK, Json(tag::get_tag_by_id(id, db).await))
}

pub async fn get_ingredients(
    Extension(claims): Extension<Option<JwtClaims>>,
    Extension(db): Extension<DbPool>,
) -> impl IntoResponse {
    assert!(claims.is_some());
    (StatusCode::OK, Json(ingredient::get_ingredients(db).await))
}

pub async fn get_recipes(
    Extension(db): Extension<DbPool>,
    Extension(claims): Extension<Option<JwtClaims>>,
) -> impl IntoResponse {
    assert!(claims.is_some());

    (
        StatusCode::OK,
        Json(recipe::Recipe::get_recipes(db).await.unwrap()),
    )
}
