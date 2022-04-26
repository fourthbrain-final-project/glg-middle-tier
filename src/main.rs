mod site_content ;

use site_content::post_requests as post_req ;
use actix_web::{get, guard, web, App, HttpServer, Responder} ;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(post_req::document_processor)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}