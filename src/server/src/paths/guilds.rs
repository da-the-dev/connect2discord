use actix_web::{get, HttpResponse, Result};
use std::{error::Error, env};

#[get("/guilds")]
pub async fn guilds() -> Result<HttpResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://discord.com/api/users/@me/guilds")
        .header("Authorization", format!("Bot {}", String::from(env::var("TOKEN")?)))
        .send()
        .await;

    match res {
        Ok(r) => {
            match r.text().await {
               Ok(t) => Ok(HttpResponse::Ok().body(t)),
               Err(e) => Ok(HttpResponse::BadGateway().body(e.to_string()))
            }
        },
        Err(e) => {
            Ok(HttpResponse::BadGateway().body(format!("There was error in the request: {}", e)))
        }
    }
}
