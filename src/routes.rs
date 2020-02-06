use json_api::controllers::{users, top};
use actix_web::web;

pub fn top(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(top::index))
        .route("/hello", web::get().to(top::hello));
}

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg.route("/name", web::get().to(users::name));
}
