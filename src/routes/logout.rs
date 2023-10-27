use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{HttpResponse, Responder};

pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
