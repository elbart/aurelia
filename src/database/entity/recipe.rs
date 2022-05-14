use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::driver::db;

use super::{ingredient::Ingredient, user::User};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

impl Recipe {
    pub async fn get_recipes(pool: db::DB) -> anyhow::Result<Vec<Recipe>> {
        let mut recipes = sqlx::query(
            r#"SELECT
          r.id AS recipe_id,
          r.name AS recipe_name,
          r.description AS recipe_description,
          r.link AS recipe_link,
          u.id AS user_id,
          u.email AS user_email,
          u.password AS user_password
        FROM recipe r
        INNER JOIN "user" u ON r.user_id = u.id
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
                ingredients: vec![],
            })
        })
        .fetch_all(&*pool)
        .await?;

        RecipeIngredient::get_recipe_ingredients(&mut recipes, pool).await?;
        Ok(recipes)
    }
}

impl RecipeIngredient {
    pub async fn get_recipe_ingredients(
        recipes: &mut Vec<Recipe>,
        pool: db::DB,
    ) -> anyhow::Result<()> {
        for r in recipes {
            let mut recipe_ingredients: Vec<RecipeIngredient> = sqlx::query(
                r#"SELECT * FROM
            recipe_ingredient
            LEFT JOIN ingredient i ON recipe_ingredient.ingredient_id = i.id
            WHERE recipe_id = $1
            "#,
            )
            .bind(r.id)
            .try_map(|row: PgRow| {
                Ok(RecipeIngredient {
                    ingredient: Ingredient {
                        id: row.try_get("id")?,
                        name: row.try_get("name")?,
                        tags: vec![],
                    },
                    quantity: row.try_get("quantity")?,
                    unit: row.try_get("unit")?,
                })
            })
            .fetch_all(&*pool)
            .await?;

            for ri in recipe_ingredients.iter_mut() {
                ri.ingredient.fetch_tags(pool.clone()).await?;
                r.ingredients.push(ri.to_owned());
            }
        }

        Ok(())
    }
}
