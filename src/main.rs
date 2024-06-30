use actix_web::{web, App, HttpServer};

mod controllers;
mod services;

use controllers::message_controller::MessageController;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/message/latest",
                web::get().to(MessageController::get_latest),
            )
            .route(
                "/message/latest/sender",
                web::get().to(MessageController::get_latest_sender),
            )
            .route(
                "/message/latest/content",
                web::get().to(MessageController::get_latest_content),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
