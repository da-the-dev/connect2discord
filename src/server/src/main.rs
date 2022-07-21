use actix_web::{get, post, web, App, HttpResponse, Result, HttpServer, Responder, http::header, body};
use actix_files as fs;
use fs::NamedFile;

async fn index() -> Result<NamedFile> {
    Ok(fs::NamedFile::open("./public/index.html")?)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Code {
    code: String
}

#[derive(Serialize, Deserialize)]
struct IdentifyRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
    code: String,
    redirect_uri: String
}

#[get("/identify")] 
async fn identify(req_body: web::Json<Code>) -> impl Responder {
    // let code = &req_body.code;
    let body = IdentifyRequest {
        client_id: String::from("997526709148598282"),
        client_secret: String::from("expunged"),
        grant_type: String::from("authorization_code"),
        code: String::from("expunged"),
        redirect_uri: String::from("http://localhost:8000/")
    };


    let client = reqwest::Client::new();
    let resp = client
        .post("https://discord.com/api/v10/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(body).unwrap())
        .send();
    match resp.await {
        Ok(r) => {
            match r.text().await {
                Ok(t) => HttpResponse::Accepted().body(t),
                Err(e) => HttpResponse::BadGateway().body(format!("Error in r.text: {}", e))
            }
        },
        Err(e) => HttpResponse::BadGateway().body(format!("Error in responce: {}", e))
    }
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