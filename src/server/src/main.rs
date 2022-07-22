use actix_web::{web, App, Result, HttpServer, HttpResponse};
use actix_files::{Files, NamedFile};
mod paths; use paths::identify::identify as identify;
use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct Code {
//     code: String
// }

// async fn test(query: web::Query<Code>) -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().body(query.code.to_string()))
// }

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./public/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            // .route("/test", web::get().to(test))
            .route("/", web::get().to(index))
            .service(identify)
            .service(Files::new("/", "./public").show_files_listing())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}