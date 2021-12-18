use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::database::DbPool;

use super::tag::Tag;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    pub tags: Vec<Tag>,
}

impl Ingredient {
    pub async fn fetch_tags(&mut self, pool: DbPool) -> anyhow::Result<()> {
        self.tags = sqlx::query(
            r#"
        SELECT t.id, t.name FROM
        ingredient_tag it
        INNER JOIN tag t ON it.tag_id = t.id
        WHERE it.ingredient_id=$1"#,
        )
        .bind(self.id)
        .try_map(|row: PgRow| {
            Ok(Tag {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
            })
        })
        .fetch_all(&*pool)
        .await?;

        Ok(())
    }
}

pub async fn get_ingredients(pool: DbPool) -> Vec<Ingredient> {
    sqlx::query("SELECT * FROM ingredient")
        .try_map(|row: PgRow| {
            Ok(Ingredient {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                tags: vec![],
            })
        })
        .fetch_all(&*pool)
        .await
        .unwrap()
}
