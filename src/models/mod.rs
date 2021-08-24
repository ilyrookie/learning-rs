use diesel::{Queryable};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub admin: bool,
}