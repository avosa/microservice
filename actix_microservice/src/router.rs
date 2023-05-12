// use actix_web::{web, App, HttpResponse, Responder};
// use actix_web::web::ServiceConfig;
// use mysql_async::Pool;
// use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct Property {
//     address: String,
//     owner: String,
// }

// pub fn property_router(app: &mut ServiceConfig) {
//     app.service(
//         web::scope("/api")
//             .route("/property", web::post().to(add_property)),
//     );
// }

// pub async fn add_property(
//     pool: web::Data<Pool>,
//     property: web::Json<Property>,
// ) -> impl Responder {
//     let mut conn = pool.get_conn().await.unwrap();
//     conn.exec_drop(
//         "INSERT INTO properties (address, owner) VALUES (?, ?)",
//         (&property.address, &property.owner),
//     )
//     .await
//     .unwrap();

//     HttpResponse::Ok().body("Property added successfully")
// }

use actix_web::{web, App};
use mysql_async::Pool;
use crate::property;

pub fn property_router(app: &mut App<actix_web::web::ServiceConfig>) {
    app.service(
        web::scope("/api")
        .route("/property", web::post().to(property::add_property)),
    );
}

// pub async fn add_property(
//     pool: web::Data<Pool>,
//     property: web::Json<Property>,
// ) -> impl Responder {
//     let mut conn = pool.get_conn().await.unwrap();
//     conn.exec_drop(
//         "INSERT INTO properties (address, owner) VALUES (?, ?)",
//         (&property.address, &property.owner),
//     )
//     .await
//     .unwrap();

//     HttpResponse::Ok().body("Property added successfully")
// }