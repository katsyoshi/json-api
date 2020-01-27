use serde::{Deserialize, Serialize};
use diesel::Queryable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub hello: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub published: bool,
}
