use actix_web::http::header::LOCATION;
use actix_web::{error, web, HttpResponse, Responder};
use askama::Template;
use derive_more::{Display, Error};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

pub async fn login_form() -> impl Responder {
    let template = LoginTemplate;
    HttpResponse::Ok().body(template.render().unwrap())
}

// TODO: think about secrey crate for password
#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: String,
}

pub async fn login_post(
    form: web::Form<FormData>,
) -> Result<HttpResponse, LoginError> {
    let username = form.0.username;
    let password = form.0.password;

    if username == "foo" && password == "bar" {
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish())
    } else {
        Err(LoginError {})
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Login Error")]
pub struct LoginError {}

impl error::ResponseError for LoginError {}
