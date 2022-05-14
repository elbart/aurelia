use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::driver::db;

use super::DbFilter;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

pub async fn get_tag_by_id(id: Uuid, pool: db::DB) -> Option<Tag> {
    sqlx::query_as("SELECT * FROM tag where id = $1")
        .bind(id)
        .fetch_optional(&*pool)
        .await
        .unwrap()
}

pub async fn get_tags(_filter: Option<DbFilter>, pool: db::DB) -> Vec<Tag> {
    sqlx::query_as("SELECT * FROM tag")
        .fetch_all(&*pool)
        .await
        .unwrap()
}
