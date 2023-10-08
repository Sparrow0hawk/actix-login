use actix_session::Session;
use actix_web::{Error, HttpResponse, Responder};
use askama::Template;

use super::LoginError;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    content_title: &'a str,
    content_main: &'a str,
}

pub async fn secure(session: Session) -> Result<impl Responder, Error> {
    if let Some(user) = session.get::<String>("user").unwrap() {
        let template = IndexTemplate {
            title: "Super secure stuff",
            content_main: "",
            content_title: &format!("Hi {}", &user),
        };
        Ok(HttpResponse::Ok().body(template.render().unwrap()))
    } else {
        Err(LoginError {}.into())
    }
}
