use repositories::message_repo;
mod model;
mod kafka;
mod db;
mod repositories;
mod routes;
mod services;
mod handlers;

use actix_web::{web::Data, App, HttpServer};
use routes::routes;
use services::message_service::MessageService;
use repositories::message_repo::MessageRepo;
use kafka::producer::EventProducer;
use kafka::consumer::EventConsumer;
use db::session::ScyllaSession;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let scylla_session = ScyllaSession::new().await?;
    let message_repo =  MessageRepo::new(scylla_session.session);

    let event_producer = EventProducer::new("192.168.0.233:9192", "counter")?;
    let event_consumer = EventConsumer::new("192.168.0.233:9192", "counter")?;


    let message_service = MessageService::new(message_repo, event_producer);
    let message_service = Data::new(message_service);

    event_consumer.run(&message_service).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(message_service.clone())
            .configure(routes)
    })
    .bind("localhost:8080")?
    .run()
    .await?;

    Ok(())
}