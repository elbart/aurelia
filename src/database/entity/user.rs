use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::database::DbPool;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

pub async fn get_user_by_id(id: Uuid, pool: DbPool) -> Option<User> {
    sqlx::query_as(r#"SELECT * FROM "user" where id = $1"#)
        .bind(id)
        .fetch_optional(&*pool)
        .await
        .unwrap()
}
