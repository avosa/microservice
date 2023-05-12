// use lapin::{Connection, ConnectionProperties};
// use tokio_amqp::LapinTokioExt;

// pub async fn connect() -> Connection {
//     let addr = "amqp://guest:guest@localhost:5672//";
//     Connection::connect(&addr, ConnectionProperties::default().with_tokio()).await.unwrap()
// }


use lapin::{Connection, ConnectionProperties};
use tokio_amqp::LapinTokioExt;

pub async fn connect() -> Connection {
    let addr = "amqp://guest:guest@localhost:5672//";
    Connection::connect(&addr, ConnectionProperties::default().with_tokio()).await.expect("Failed to connect to RabbitMQ")
}
