use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    content_title: &'a str,
    content_main: &'a str,
}

pub async fn index() -> impl Responder {
    let template = IndexTemplate {
        title: "Hello world",
        content_main: "",
        content_title: "Hello world!",
    };
    HttpResponse::Ok().body(template.render().unwrap())
}
