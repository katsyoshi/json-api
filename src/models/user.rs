use super::util::establish_connection;
use crate::schema::users::dsl;

use diesel::Queryable;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub published: bool,
}

impl User {
    pub fn find_by_name(user_name: String) -> User {
        let conn = establish_connection();
        dsl::users
            .filter(dsl::name.eq(user_name))
            .filter(dsl::published.eq(true))
            .first::<User>(&conn)
            .expect("Error finding user")
    }
}
