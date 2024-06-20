pub async fn catalog_response_body(query: &str) -> String {
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
#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn catalog_response_body_works() {
        let result= catalog_response_body("cats")
        .await;
        assert!(result.contains("cats"));
    }
}
