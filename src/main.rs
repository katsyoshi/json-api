#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use std::env;

pub mod schema;
pub mod models;

use models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!(
        "Error connecting to {}",
        database_url
    ))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?wave&<name>")]
fn hello(name: &RawStr) -> Json<Hello> {
    Json(Hello { hello: name.as_str().to_string() })
}

#[get("/name?<user_name>")]
fn user(user_name: &RawStr) -> Json<User> {
    use schema::users::dsl::*;
    let conn = establish_connection();
    let u = users
        .filter(name.eq(user_name.to_string()))
        .filter(published.eq(true))
        .first::<User>(&conn)
        .expect("Error finding posts");
    Json(u)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, user])
        .launch();
}
