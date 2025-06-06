use actix_web::{
    web,
    HttpResponse, Responder,
};
use uuid::Uuid;
use crate::model::message::{Event, GetViewResponse, Message, Payload, PostResponse};
use crate::services::message_service::MessageService;


pub async fn produce_message(service: web::Data<MessageService>, payload: web::Json<Event>) -> impl Responder {
    match service.produce_message(payload.into_inner()).await {
        Ok(message) => 
            HttpResponse::Created().json(PostResponse {
                result: true,
            }),
        Err(e) => HttpResponse::InternalServerError().body("Failed to create"),
    }
}   

pub async fn get_viewers(service: web::Data<MessageService>, payload: web::Json<String>) -> impl Responder {
    match service.get_viewers(payload.into_inner()).await {
        Ok(viewers_count) => 
            HttpResponse::Ok().json(GetViewResponse {   
                viewers: viewers_count,
            }),
        Err(e) => {
    // Log or return the actual error string so you know exactly what went wrong:
    let msg = format!("Failed to get viewers: {}", e);
    HttpResponse::InternalServerError().body(msg)
  }}
}

// pub async fn update_message(service: web::Data<MessageService>, payload: web::Json<Payload>) -> impl Responder {
//     match service.create_message(payload.into_inner()).await {
//         Ok(message) => HttpResponse::Ok().json(message),
//         Err(e) => HttpResponse::InternalServerError().body(e),
//     }
// }