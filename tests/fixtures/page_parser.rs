use scraper::Selector;

pub fn get_page_element(body: &str, element: &str) -> String {
    let fragment = scraper::Html::parse_document(body);

    let selector = Selector::parse(element).unwrap();

    fragment.select(&selector).next().unwrap().inner_html()
}
