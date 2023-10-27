mod fixtures;

use fixtures::spawn_app;

#[actix_web::test]
async fn test_logout_deletes_cookie() {
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

    // test /logout
    let visit_logout = client
        .get(&format!("{}/logout", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(visit_logout.cookies().next().is_none())
}
