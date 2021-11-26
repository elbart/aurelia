use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

use crate::database::DbPool;

use super::{ingredient::Ingredient, user::User};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecipeIngredient {
    inner: Vec<Ingredient>, // quantity: IngredientQuantity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub user_id: Uuid,
    pub user: User,
}

pub async fn get_recipes(pool: DbPool) -> Vec<Recipe> {
    let mut recipes = sqlx::query(
        r#"SELECT recipe.id as recipe_id, recipe.name, recipe.description, recipe.link, recipe.user_id, "user".id as user_id, "user".email, "user".password FROM recipe LEFT JOIN "user" ON recipe.user_id = "user".id"#,
    )
    .try_map(|row: PgRow| {
        Ok(Recipe {
            id: row.try_get("recipe_id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            link: row.try_get("link")?,
            user_id: row.try_get("user_id")?,
            user: User {
                id: row.try_get("user_id")?,
                email: row.try_get("email")?,
                password: row.try_get("password")?
            }
        })
    })
    .fetch_all(&*pool)
    .await
    .unwrap();

    recipes
}
