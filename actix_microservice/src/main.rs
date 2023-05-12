mod db;
mod rabbitmq;
mod property;
mod router;

use actix_web::{web, App, HttpServer};
use futures_util::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to MySQL
    let pool = db::connect().await;

    // Create Actix web server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(router::property_router)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    // Connect to RabbitMQ
    let conn = rabbitmq::connect().await;
    let channel = conn.create_channel().await.unwrap();

    let _queue = channel
        .queue_declare(
            "userCreated",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let mut consumer = channel
        .basic_consume(
            "userCreated",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("ack");

        // Handle user creation event
        let user = std::str::from_utf8(&delivery.data).unwrap();
        println!("Received user: {}", user);
    }

    Ok(())
}
