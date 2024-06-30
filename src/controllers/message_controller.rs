use actix_web::{HttpResponse, Responder};

use crate::services::message_service::MessageService;

pub struct MessageController;

impl MessageController {
    pub async fn get_latest() -> impl Responder {
        let message = MessageService::get_latest();

        HttpResponse::Ok().json(message)
    }

    pub async fn get_latest_sender() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    pub async fn get_latest_content() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
}
