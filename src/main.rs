#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket_contrib::json::Json;

use json_api::models::User;
use json_api::models::hello::Hello;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?<name>")]
fn hello(name: &RawStr) -> Json<Hello> {
    Json(Hello { hello: name.to_string() })
}

#[get("/name?<user_name>")]
fn user(user_name: &RawStr) -> Json<User> {
    let u = User::find_by_name(user_name.to_string());
    Json(u)
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello, user]).launch();
}
