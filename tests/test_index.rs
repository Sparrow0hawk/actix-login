use actix_login::routes::index;
use actix_web::{http::header::ContentType, test, web, App};

#[actix_web::test]
async fn test_index() {
    let app =
        test::init_service(App::new().route("/", web::get().to(index))).await;

    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success())
}
