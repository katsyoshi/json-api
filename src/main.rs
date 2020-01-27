#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket_contrib::json::Json;

mod models;
use models::Hello;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?wave&<name>")]
fn hello(name: &RawStr) -> Json<Hello> {
    Json(Hello { hello: name.as_str().to_string(), })
}

fn main() {
    rocket::ignite().mount("/", routes![index,hello]).launch();
}

