pub mod ingredient;
pub mod recipe;
pub mod tag;
pub mod user;

pub struct DbFilter {
    limit: Option<usize>,
}
