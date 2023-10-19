#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod ads;
mod assets;

mod misc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // API v1 
            .service(web::redirect("/v1", "/v1/info"))
            .service(web::redirect("/v1/", "/v1/info"))
            .service(misc::index)
            .service(misc::v1_info)
            .service(ads::v1_ads_random)
            .service(assets::v1_assets_random)
    })
    .bind(("127.0.0.1", 3010))?
    .run()
    .await
}