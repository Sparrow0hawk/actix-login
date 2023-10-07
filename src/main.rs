use std::net::TcpListener;

use actix_login::startup::run;

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = actix_login::cli::parse_args().expect("Error parsing arguments");

    let address = format!("{}:{}", args.host, args.port);
    println!("App listening on http://{}", &address);

    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
