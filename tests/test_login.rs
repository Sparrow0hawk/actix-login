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

    assert_eq!(resp.headers().get("LOCATION").unwrap(), "/secure")
}

#[actix_web::test]
async fn test_logged_in_visit_to_login_redirects() {
    let app = spawn_app().await;

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let payload = &serde_json::json!({
        "username": "foo",
        "password": "bar"
    });

    // login
    let _login_resp = client
        .post(&format!("{}/login", &app.address))
        .form(payload)
        .send()
        .await
        .expect("Failed to execute request");

    // test /login
    let visit_login = client
        .get(&format!("{}/login", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let page_str = &visit_login.text().await.unwrap();

    let page = get_page_element(page_str, "h1");

    assert_eq!(page, "Hello world!")
}
