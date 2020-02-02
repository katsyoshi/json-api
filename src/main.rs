#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket_contrib::json::Json;


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
