use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header, body};
use actix_files as fs;
use fs::NamedFile;
use actix_web::Result;

async fn index() -> Result<NamedFile> {
    Ok(fs::NamedFile::open("./public/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", "./public").show_files_listing())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}