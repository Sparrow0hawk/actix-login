use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    content_title: &'a str,
    content_main: &'a str,
    user: Option<String>,
}

pub async fn index(session: Session) -> impl Responder {
    let current_user = session.get::<String>("user").unwrap();

    let template = IndexTemplate {
        title: "Hello world",
        content_main: "",
        content_title: "Hello world!",
        user: current_user,
    };
    HttpResponse::Ok().body(template.render().unwrap())
}
