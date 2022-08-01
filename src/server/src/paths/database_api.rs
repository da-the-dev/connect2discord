use actix_web::{post, HttpResponse, web};
use serde_json::to_string;
use std::{env, error::Error};
use eyre::Result;
use serde::{Deserialize, Serialize};
use couch_rs::{types::{document::DocumentId, find::FindQuery}, CouchDocument, document::TypedCouchDocument, Client};

const DB_HOST: &str = "http://localhost:5984";
const DB_NAME: &str = "connect2discord";
const DB_LOGIN: &str = "admin";

#[derive(Serialize, Deserialize, CouchDocument)]
pub struct GuildSettings {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    #[serde(rename = "embedColor")]
    pub embed_color: String
}


#[post("/db/saveSettings")]
pub async fn save_settings(raw_settings: web::Json<GuildSettings>) -> Result<HttpResponse, Box<dyn Error>> {
    let client = Client::new(DB_HOST, DB_LOGIN, &env::var("CDB_PASS")?)?;
    let db = client.db(DB_NAME).await?;

    let mut settings: GuildSettings = raw_settings.into_inner();
    println!("{}", to_string(&settings)?);
    let result = db.save(&mut settings).await;

    match result {
        Ok(_r) => {
            let new_settings: GuildSettings = db.get(&settings._id).await?;

            Ok(HttpResponse::Accepted().insert_header(("Content-Type", "application-json")).body(to_string(&new_settings)?))
        },
        Err(e) => Ok(HttpResponse::BadRequest().body(e.message))
    }
    
}

