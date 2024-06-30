use actix_web::{HttpResponse, Responder};

use crate::services::message_service::MessageService;

pub struct MessageController;

impl MessageController {
    pub async fn get_latest() -> impl Responder {
        let message = MessageService::get_latest();

        HttpResponse::Ok().json(message)
    }

    pub async fn get_latest_sender() -> impl Responder {
        let message = MessageService::get_latest_sender();

        HttpResponse::Ok().json(message)
    }

    pub async fn get_latest_content() -> impl Responder {
        let message = MessageService::get_latest_content();

        HttpResponse::Ok().json(message)
    }
}
