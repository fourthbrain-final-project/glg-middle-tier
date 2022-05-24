mod site_content ;
mod tls_config ;

use actix_cors::Cors ;
use actix_web::{get, web, App, HttpServer, Responder, http, middleware::Logger} ;
use env_logger ;
use site_content::post_requests as post_req ;

use std::env ;

use tls_config::load_rustls_config ;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display()) ;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let ssl_conf = load_rustls_config("certs/cert.pem", "certs/key.pem") ;

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::ORIGIN])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600) ;
            
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(post_req::classify_document)
            .service(post_req::topic_generator)
            .service(post_req::entity_extractor)

    })
    .bind_rustls("127.0.0.1:8443", ssl_conf)?
    .run()
    .await
}