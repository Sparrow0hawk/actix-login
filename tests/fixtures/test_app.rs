use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server =
        actix_login::startup::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp { address }
}
