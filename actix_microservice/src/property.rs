// use actix_web::{web, HttpResponse, Responder};
// use mysql_async::{prelude::Queryable, Pool};
// use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct Property {
//     address: String,
//     owner: String,
// }

// pub async fn add_property(pool: web::Data<Pool>, property: web::Json<Property>) -> impl Responder {
//     let mut conn = pool.get_conn().await.unwrap();
//     conn.exec_drop(
//         "INSERT INTO properties (address, owner) VALUES (?, ?)",
//         (&property.address, &property.owner),
//     )
//     .await
//     .unwrap();

//     HttpResponse::Ok().body("Property added successfully")
// }

use actix_web::{web, HttpResponse, Responder};
use mysql_async::{prelude::Queryable, Pool};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Property {
    address: String,
    owner: String,
}

pub async fn add_property(pool: web::Data<Pool>, property: web::Json<Property>) -> impl Responder {
    let mut conn = pool.get_conn().await.expect("Failed to get connection from pool");
    conn.exec_drop(
        "INSERT INTO properties (address, owner) VALUES (?, ?)",
        (&property.address, &property.owner),
    )
    .await
    .expect("Failed to insert property");

    HttpResponse::Ok().body("Property added successfully")
}
