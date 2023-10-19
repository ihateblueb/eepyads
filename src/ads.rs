#[allow(unused_imports)]
use std::env;
use actix_web::web::Json;
use serde::Serialize;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};

#[derive(Serialize)]
struct ad_obj {
    status: i32,
    ad_id: i32,
    ad_url: String
}

#[get("/v1/ads/random")]
pub async fn v1_ads_random() -> impl Responder {
    let ad_output = ad_obj {
        status: 200,
        ad_id: 0,
        ad_url: "".to_string()
    };

    println!("[ADS] [GET /v1/ads/random] <--> {}; {}; {};", "STATUS=".to_string()+&ad_output.status.to_string(), "ID=".to_string()+&ad_output.ad_id.to_string(),"URL=".to_string()+&ad_output.ad_url);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("X-EepyAdsVersion", env!("CARGO_PKG_VERSION")))
        .json(web::Json(ad_output))
}