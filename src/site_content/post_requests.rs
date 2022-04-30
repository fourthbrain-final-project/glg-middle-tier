use actix_web::{post, web, Responder, Result} ;
use serde::{Deserialize, Serialize} ;
use uuid::Uuid ;

#[derive(Serialize, Deserialize)]
pub struct Document {
    document_content: String
}

#[derive(Serialize, Deserialize)]
pub struct PostSuccess {
    code: u16,
    message: String,
    job_id: String ,
}

#[post("/document")]
pub async fn document_processor(doc: web::Json<Document>) -> Result<impl Responder> {
    let id = Uuid::new_v4() ;
    let status = PostSuccess {
        code: 201,
        message : "Success".into(),
        job_id: id.to_string() ,
    } ;

    Ok(web::Json(status))
}