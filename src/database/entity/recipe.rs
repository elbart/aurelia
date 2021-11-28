use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::database::DbPool;

use super::{ingredient::Ingredient, user::User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeIngredient {
    pub ingredient: Ingredient,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub user_id: Uuid,
    pub user: User,
    pub ingredients: Vec<RecipeIngredient>,
}

pub async fn get_recipes(pool: DbPool) -> anyhow::Result<Vec<Recipe>> {
    let recipes = sqlx::query(
        r#"SELECT
          r.id AS recipe_id,
          r.name AS recipe_name,
          r.description AS recipe_description,
          r.link AS recipe_link,
          u.id AS user_id,
          u.email AS user_email,
          u.password AS user_password,
          i.id AS ingredient_id,
          i.name AS ingredient_name,
          ri.unit AS ri_unit,
          ri.quantity AS ri_quantity
        FROM recipe r
        INNER JOIN "user" u ON r.user_id = u.id
        LEFT JOIN recipe_ingredient ri ON r.id = ri.recipe_id
        LEFT JOIN ingredient i ON ri.ingredient_id = i.id
        "#,
    )
    .try_map(|row: PgRow| {
        let ri = if let Some(id) = row.try_get("ingredient_id")? {
            vec![RecipeIngredient {
                ingredient: Ingredient {
                    id,
                    name: row.try_get("ingredient_name")?,
                },
                quantity: row.try_get("ri_quantity")?,
                unit: row.try_get("ri_unit")?,
            }]
        } else {
            vec![]
        };

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
            ingredients: ri,
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
