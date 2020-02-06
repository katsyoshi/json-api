use crate::models::User;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn name(req: HttpRequest) -> impl Responder {
    let user = find_user(req);
    let res = serde_json::to_string(&user).unwrap();

    HttpResponse::Ok().body(res)
}

fn find_user(req: HttpRequest) -> User {
    let user_name: String = qstring::QString::from(req.query_string()).get("user_name").unwrap().to_string();
    User::find_by_name(user_name)
}
