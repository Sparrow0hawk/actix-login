use actix_login::routes::index;
use actix_web::{http::header::ContentType, test, web, App};
use scraper::Selector;

#[actix_web::test]
async fn test_index() {
    let app =
        test::init_service(App::new().route("/", web::get().to(index))).await;

    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_and_read_body(&app, req).await;

    let page_str = std::str::from_utf8(&resp).unwrap();

    let page = get_page_element(page_str, "h1");

    assert_eq!(page, "Hello world!")
}

fn get_page_element(body: &str, element: &str) -> String {
    let fragment = scraper::Html::parse_document(body);

    let selector = Selector::parse(element).unwrap();

    fragment.select(&selector).next().unwrap().inner_html()
}
