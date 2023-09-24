use std::net::TcpListener;

use crate::routes::index;

use actix_files as fs;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(fs::Files::new("/static", "./static"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
