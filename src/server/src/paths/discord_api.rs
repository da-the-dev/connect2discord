use actix_web::{get, HttpResponse, web::Path, HttpResponseBuilder};
use reqwest::{Client, StatusCode, Response};
use serde::Deserialize;
use std::{env, time::Duration, error::Error};
use tokio::time;
use std::future::Future;

#[derive(Deserialize)]
struct DiscordRateLimitError {
    message: String,
    retry_after: f32,
    global: bool
}

#[get("/botapi/{tail:.*}")]
pub async fn discord_api(path: Path<String>) -> HttpResponse {
    let endpoint = path.into_inner();
    let client = Client::new();
    let response = client
        .get(format!("https://discord.com/api/{}", &endpoint))
        .header("Authorization", format!("Bot {}", env::var("TOKEN").unwrap()))
        .send()
        .await.unwrap();

    async fn regen_request(bad_response: Response, endpoint: &str) -> Response {
        let error: DiscordRateLimitError = serde_json::from_str(&bad_response.text().await.unwrap()).unwrap();
        time::sleep(Duration::from_secs_f32(error.retry_after)).await;

        let client = Client::new();
        client
            .get(format!("https://discord.com/api/{}", endpoint))
            .header("Authorization", format!("Bot {}", env::var("TOKEN").unwrap()))
            .send()
            .await.unwrap()
    }

    async fn send_response(response: Response) -> HttpResponse { 
        let mut actix_response = HttpResponseBuilder::new(*&response.status());
        for header in response.headers().iter() {
            actix_response.append_header(header);
        }
        
        actix_response.body(response.text().await.unwrap().to_string())
    }

    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        send_response(regen_request(response, &endpoint).await).await
    } else {
        send_response(response).await
    }
}
