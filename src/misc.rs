#[allow(unused_imports)]
use std::env;
use actix_web::web::Json;
use serde::Serialize;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};

#[get("/")]
pub async fn index() -> impl Responder {
    println!("[ADS] [GET /] <--> Sent homepage");

    HttpResponse::Ok()
        .body("eepy ads... https://eepy.zone/notes/9kylwhckqaz6z3qd")
}

#[derive(Serialize)]
struct misc_obj {
    status: i32,
    version: String,
    id: i32
}

#[get("/v1/info")]
pub async fn v1_info() -> impl Responder {
    let misc_output = misc_obj {
        status: 200,
        version: env!("CARGO_PKG_VERSION").to_string(),
        id: 0,
    };

    println!("[ADS] [GET /v1/info] <--> {}; {}; {};", "STATUS=".to_string()+&misc_output.status.to_string(), "VERSION=".to_string()+&misc_output.version.to_string(),"ID=".to_string()+&misc_output.id.to_string());

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("X-EepyAdsVersion", env!("CARGO_PKG_VERSION")))
        .json(web::Json(misc_output))
}