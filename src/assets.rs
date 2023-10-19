use std::path::Path;

use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType, ContentType};
use actix_web::{get, App, Error, HttpRequest, HttpServer, HttpResponse};

#[get("/v1/assets/random/{filename:.*}")]
async fn v1_assets_random(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    println!("[ADS] [GET /v1/assets/random/{}] <--> {}", req.match_info().query("filename").parse::<String>().unwrap(), "./src/assets/random/".to_string()+&req.match_info().query("filename").parse::<String>().unwrap());

    let path = Path::new("./src/assets/random/").join(req.match_info().query("filename").parse::<String>().unwrap());
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {disposition: DispositionType::Attachment,parameters: vec![]})
    )
}
