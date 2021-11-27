use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

use crate::database::DbPool;

use super::{ingredient::Ingredient, user::User};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecipeIngredient {
    inner: Vec<Ingredient>, // quantity: IngredientQuantity,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub user_id: Uuid,
    pub user: User,
    pub ingredients: Vec<Ingredient>,
}

pub async fn get_recipes(pool: DbPool) -> anyhow::Result<Vec<Recipe>> {
    let recipes = sqlx::query(
        r#"SELECT
          r.id AS recipe_id,
          r.name AS recipe_name,
          r.description AS recipe_description,
          r.link as recipe_link,
          u.id as user_id,
          u.email as user_email,
          u.password as user_password,
          i.id as ingredient_id,
          i.name as ingredient_name
        FROM recipe r
        INNER JOIN "user" u ON r.user_id = u.id
        INNER JOIN recipe_ingredient ri ON r.id = ri.recipe_id
        INNER JOIN ingredient i ON ri.ingredient_id = i.id
        "#,
    )
    .try_map(|row: PgRow| {
        Ok(Recipe {
            id: row.try_get("recipe_id")?,
            name: row.try_get("recipe_name")?,
            description: row.try_get("recipe_description")?,
            link: row.try_get("recipe_link")?,
            user_id: row.try_get("user_id")?,
            user: User {
                id: row.try_get("user_id")?,
                email: row.try_get("user_email")?,
                password: row.try_get("user_password")?,
            },
            ingredients: vec![Ingredient {
                id: row.try_get("ingredient_id")?,
                name: row.try_get("ingredient_name")?,
            }],
        })
    })
    .fetch_all(&*pool)
    .await?
    .iter()
    .fold(Vec::new(), |mut recipes: Vec<Recipe>, recipe: &Recipe| {
        if let Some(r) = recipes.last_mut() {
            if r.id == recipe.id {
                r.ingredients.extend(recipe.ingredients.clone());
            }
        } else {
            recipes.push(recipe.clone());
        }
        recipes
    });

    Ok(recipes)
}
