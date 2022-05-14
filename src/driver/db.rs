use sqlx::{Executor, Pool, Postgres, Transaction};

pub type DB = Pool<Postgres>;
pub trait Queryer<'c>: Executor<'c, Database = sqlx::Postgres> {}
impl<'c> Queryer<'c> for &Pool<Postgres> {}
impl<'c> Queryer<'c> for &'c mut Transaction<'_, Postgres> {}
