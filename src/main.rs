use actix_web::{get, post};
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

use mongodb::{bson::doc, error::Result as mongoError, Client};

#[get("/HandleOn/{roomID}/{ButtonID}")]
async fn HandleOn(v: web::Path<(u32, u32)>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Got RoomID {} and Button ID {} to turn On",
        v.0, v.1
    ))
}

#[get("/HandleOff{roomID}/{ButtonID}")]
async fn HandleOff(v: web::Path<(u32, u32)>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Got RoomID {} and Button ID {} to turn Off",
        v.0, v.1
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(HandleOn).service(HandleOff))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
