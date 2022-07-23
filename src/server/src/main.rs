use actix_web::{web, App, Result, HttpServer};
use actix_files::{Files, NamedFile};
use actix_cors::Cors;
mod paths; use paths::identify::identify as identify;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("../../../public/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .service(identify)
            .service(Files::new("/", "../../../public").show_files_listing())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}