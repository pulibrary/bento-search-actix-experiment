pub async fn catalog_response_body(query: &String) -> String {
    let catalog_url = format!(
        "https://catalog.princeton.edu/catalog.json?utf8=%E2%9C%93&search_field=all_fields&q={}",
        query
    );
    let body = reqwest::get(catalog_url)
    .await.unwrap()
    .text()
    .await.unwrap();
    body
}
