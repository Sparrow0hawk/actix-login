use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{error, web, Error, HttpResponse, Responder};
use askama::Template;
use derive_more::{Display, Error};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

pub async fn login_form(session: Session) -> impl Responder {
    if let Some(_user) = session.get::<String>("user").unwrap() {
        HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish()
    } else {
        let template = LoginTemplate;
        HttpResponse::Ok().body(template.render().unwrap())
    }
}

// TODO: think about secrey crate for password
#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: String,
}

pub async fn login_post(
    form: web::Form<FormData>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let username = form.0.username;
    let password = form.0.password;

    if username == "foo" && password == "bar" {
        session.insert("user", "foo")?;
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/secure"))
            .finish())
    } else {
        Err(LoginError {}.into())
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Login Error")]
pub struct LoginError {}

impl error::ResponseError for LoginError {}
