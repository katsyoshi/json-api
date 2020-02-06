use crate::models::hello::Hello;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn hello(req: HttpRequest) -> impl Responder {
    let world = find_world(req);
    let res = serde_json::to_string(&world).unwrap();

    HttpResponse::Ok().body(res)
}

fn find_world(req: HttpRequest) -> Hello {
    let user_name: String = qstring::QString::from(req.query_string()).get("name").unwrap().to_string();
    Hello{hello: user_name}
}
