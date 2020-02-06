#![feature(custom_inner_attributes)]
extern crate json_api;

mod routes;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(
        move || {
            App::new()
                .configure(routes::users)
                .configure(routes::top)
        })
        .workers(64)
        .bind("0.0.0.0:8001")
        .unwrap()
        .run()
        .await
}
