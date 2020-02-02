use json_api::models::User;
use json_api::models::hello::Hello;

use rocket::get;
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?<name>")]
pub fn hello(name: &RawStr) -> Json<Hello> {
    Json(Hello { hello: name.to_string() })
}

#[get("/name?<user_name>")]
pub fn user(user_name: &RawStr) -> Json<User> {
    let u = User::find_by_name(user_name.to_string());
    Json(u)
}
