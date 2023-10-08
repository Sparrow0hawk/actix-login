mod fixtures;

use fixtures::spawn_app;

#[actix_web::test]
async fn test_secure_returns_500_with_no_login() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/secure", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(resp.status(), 500)
}

#[actix_web::test]
async fn test_secure_returns_200_when_logged_in() {
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

    // test /secure
    let secure_resp = client
        .get(&format!("{}/secure", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(secure_resp.status(), 200)
}
