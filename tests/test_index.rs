mod fixtures;

use fixtures::{get_page_element, spawn_app};

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
