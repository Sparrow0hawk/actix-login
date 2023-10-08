use std::net::TcpListener;

use crate::routes::{index, login_form, login_post, secure};

use actix_files as fs;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let secret_key = Key::generate();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login_post))
            .route("/secure", web::get().to(secure))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(fs::Files::new("/static", "./static"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
