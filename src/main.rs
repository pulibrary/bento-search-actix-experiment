use actix_web::{get, post, web, App, http::header::ContentType, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[get("/catalog")]
async fn catalog(query: web::Query<QueryParams>) -> impl Responder {
    let catalog_url = format!(
        "https://catalog.princeton.edu/catalog.json?utf8=%E2%9C%93&search_field=all_fields&q={}",
        query.query
    );
    let body = reqwest::get(catalog_url)
    .await.unwrap()
    .text()
    .await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(catalog)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
