use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

pub async fn login() -> impl Responder {
    let template = LoginTemplate;
    HttpResponse::Ok().body(template.render().unwrap())
}
