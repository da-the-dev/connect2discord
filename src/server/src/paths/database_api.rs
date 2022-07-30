#![allow(non_snake_case)]
use std::error::Error;

use actix_web::{get, web::Path, HttpResponse};
use couch_rs::{types::{document::DocumentId}, CouchDocument, error::CouchResult};
use serde::{Serialize, Deserialize};
use couch_rs::document::TypedCouchDocument;

const DB_HOST: &str = "http://localhost:5984";
const DB_NAME: &str = "connect2discord";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Settings {
    pub embedColor: String
}

#[derive(Serialize, Deserialize, CouchDocument, Default, Debug)]
pub struct GuildEntry {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,

    #[serde(flatten)]
    pub settings: Settings
}

#[get("/db/get/{id}")]
pub async fn db_get_settings(path: Path<String>) -> Result<HttpResponse, Box<dyn Error>> {
    let id = path.into_inner();
    
    let client = couch_rs::Client::new_no_auth(DB_HOST)?;
    let db = client.db(DB_NAME).await?;

    let response: CouchResult<GuildEntry> = db.get(&id).await;
    if let Ok(db_document) = response {
        let mut response = HttpResponse::Ok();
        response.append_header(("Content-Type", "application/json"));
        Ok(response.body(serde_json::to_string(&db_document.settings)?))
    } else {
        Ok(HttpResponse::BadRequest().body("No query found"))
    }
} 
