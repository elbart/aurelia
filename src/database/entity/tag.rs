use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::database::DbPool;

use super::DbFilter;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    id: Uuid,
    name: String,
}

pub async fn get_tag_by_id(id: Uuid, pool: DbPool) -> Option<Tag> {
    sqlx::query_as("SELECT * FROM tag where id = $1")
        .bind(id)
        .fetch_optional(&*pool)
        .await
        .unwrap()
}

pub async fn get_tags(_filter: Option<DbFilter>, pool: DbPool) -> Vec<Tag> {
    sqlx::query_as("SELECT * FROM tag")
        .fetch_all(&*pool)
        .await
        .unwrap()
}
