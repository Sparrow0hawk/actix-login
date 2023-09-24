use std::net::TcpListener;

use scraper::Selector;

pub struct TestApp {
    pub address: String,
}

#[actix_web::test]
async fn test_index() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let page_str = &resp.text().await.unwrap();

    let page = get_page_element(page_str, "h1");

    assert_eq!(page, "Hello world!")
}

async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server =
        actix_login::startup::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp { address }
}

fn get_page_element(body: &str, element: &str) -> String {
    let fragment = scraper::Html::parse_document(body);

    let selector = Selector::parse(element).unwrap();

    fragment.select(&selector).next().unwrap().inner_html()
}
