use actix_web::{web, App, Result, HttpServer};
use actix_files::{Files, NamedFile};

mod paths;
use crate::paths::identify::identify;
use crate::paths::discord_api::discord_api;
use crate::paths::database_api::db_get_settings;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("../../../public/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Server starts on http://localhost:8000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(identify)
            .service(discord_api)
            .service(db_get_settings)
            .service(Files::new("/", "../../../public").show_files_listing())
        })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
