use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::DbPool;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IngredientQuantity {
    Piece(usize),
    Gram(usize),
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    // tags: Tags,
}

pub async fn get_ingredients(pool: DbPool) -> Vec<Ingredient> {
    sqlx::query_as("SELECT * FROM ingredient")
        .fetch_all(&*pool)
        .await
        .unwrap()
}
