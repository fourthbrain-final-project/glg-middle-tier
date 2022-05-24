use actix_web::{post, web, Responder, Result} ;
use reqwest::{Client} ;
use serde::{Deserialize, Serialize} ;
use uuid::Uuid ;

#[derive(Serialize, Deserialize)]
pub struct Document {
    document: String
}

#[derive(Serialize, Deserialize)]
pub struct TopicInput {
    document: String ,
    topic: String ,
}

#[derive(Serialize, Deserialize)]
pub struct TopicOutput {
    labels: Vec<String>,
    scores: Vec<f32>
}

#[derive(Serialize, Deserialize)]
pub struct ClassifyOutput {
    label: String
}

#[derive(Serialize, Deserialize)]
pub struct NamedEntityOutput {
    entities: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct PostSuccess {
    code: u16,
    message: String,
    job_id: String ,
}

/*
#[post("/document")]
pub async fn document_processor(doc: web::Json<Document>) -> Result<impl Responder> {
    let id = Uuid::new_v4() ;
    let status = PostSuccess {
        code: 201,
        message : "Success".into(),
        job_id: id.to_string() ,
    } ;

    Ok(web::Json(status))
}*/

#[post("/classify")]
pub async fn classify_document(doc: web::Json<Document>) -> Result<impl Responder> {
    let client = Client::new() ;

    //let new_doc: Document = Document { document: doc.document.to_string() } ;

    let res = client.post("http://127.0.0.1:8000/classify")
        .json(&doc)
        .send()
        .await
        .unwrap()
        .json::<ClassifyOutput>()
        .await ;
    
    let content: ClassifyOutput = match res{
        Ok(s) => s,
        Err(_s) => {
            ClassifyOutput { label: "none".into()}
        }
    } ;
    
    Ok(web::Json(content))
}
#[post("/topics")]
pub async fn topic_generator(topic: web::Json<TopicInput>) -> Result<impl Responder> {
    let client = Client::new() ;

    let res = client.post("http://127.0.0.1:8000/topics")
        .json(&topic)
        .send()
        .await
        .unwrap()
        .json::<TopicOutput>()
        .await ;
    
    let content: TopicOutput = match res {
        Ok(s) => s,
        Err(_s) => {
            println!("{:?}", _s) ;
            TopicOutput {labels: vec![], scores: vec![]}
        }
    };

    Ok(web::Json(content))
}

#[post("/entities")]
pub async fn entity_extractor(doc: web::Json<Document>) -> Result<impl Responder> {
    let client = Client::new() ;

    let res = client.post("http://127.0.0.1:8000/entities")
        .json(&doc)
        .send()
        .await
        .unwrap()
        .json::<NamedEntityOutput>()
        .await ;
    
    let content: NamedEntityOutput = match res {
        Ok(s) => s,
        Err(_s) => {
            println!("{:?}", _s) ;
            NamedEntityOutput {entities: vec![]}
        }
    };

    Ok(web::Json(content))
}
