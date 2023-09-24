use std::net::TcpListener;

use actix_login::startup::run;

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let address = "0.0.0.0:8080".to_string();
    println!("App listening on http://{}", &address);

    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
