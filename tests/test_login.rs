mod fixtures;

use fixtures::{get_page_element, spawn_app};
use reqwest::Client;

#[actix_web::test]
async fn test_login_form_page_renders() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/login", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let page_str = &resp.text().await.unwrap();

    let page = get_page_element(page_str, "h1");

    assert_eq!(page, "Log In")
}

#[actix_web::test]
async fn test_login_form_errors() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let payload = &serde_json::json!({
        "username": "blue",
        "password": "onion"
    });

    let resp = client
        .post(&format!("{}/login", &app.address))
        .form(payload)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(resp.status(), 500)
}

#[actix_web::test]
async fn test_login_form_logs_in() {
    let app = spawn_app().await;

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Error building reqwest client");

    let payload = &serde_json::json!({
        "username": "foo",
        "password": "bar"
    });

    let resp = client
        .post(&format!("{}/login", &app.address))
        .form(payload)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(resp.headers().get("LOCATION").unwrap(), "/")
}
