use actix_web::{post, web, HttpResponse} ;
use serde::{Deserialize, Serialize} ;

#[derive(Serialize, Deserialize)]
pub struct Document {
    document_content: String
}

#[post("/document")]
pub async fn document_processor(doc: web::Json<Document>) -> HttpResponse {
    print!("Document content {}", &doc.document_content);
    HttpResponse::Ok().body("document created") 
}