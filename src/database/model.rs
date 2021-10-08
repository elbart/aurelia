use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[crud_table(formats_pg:"id:{}::uuid")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}
