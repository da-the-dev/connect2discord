use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::{error::Error, env};
use serde_urlencoded::to_string;

#[derive(Serialize, Deserialize)]
pub struct Code {
    code: String
}
#[derive(Serialize, Deserialize)]
pub struct IdentifyRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
    code: String,
    redirect_uri: String
}

#[get("/identify")]
pub async fn identify(info: web::Query<Code>) -> Result<HttpResponse, Box<dyn Error>> {
    // Create the request to get user access code
    let code = &info.code;
    let body = IdentifyRequest {
        client_id: String::from("997526709148598282"),
        client_secret: env::var("CLIENT_SECRET")?,
        grant_type: String::from("authorization_code"),
        code: code.to_string(),
        redirect_uri: String::from("http://localhost:8000/")
    };
    // Complete the request
    let client = reqwest::Client::new();
    let resp = client
        .post("https://discord.com/api/v10/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(to_string(body)?)
        .send();

    match resp.await {
        Ok(r) => {
            match r.text().await {
                Ok(t) => {
                    Ok(HttpResponse::Ok().body(t))
                },
                Err(e) => Ok(HttpResponse::BadGateway().body(format!("Error in r.text: {}", e)))
            }
        },
        Err(e) => Ok(HttpResponse::BadGateway().body(format!("Error in responce: {}", e)))
    }
}
