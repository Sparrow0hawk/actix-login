use actix_login::routes::index;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
