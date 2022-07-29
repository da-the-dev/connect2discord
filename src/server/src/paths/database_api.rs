#![allow(non_snake_case)]
use std::error::Error;

use actix_web::{get, web::Path, HttpResponse};
use couch_rs::{types::{find::FindQuery, document::DocumentId}, document::DocumentCollection, CouchDocument};
use serde::{Serialize, Deserialize};
use serde_json::json;
use couch_rs::document::TypedCouchDocument;
use serde_json::to_string as json2string;

const DB_HOST: &str = "http://localhost:5984";
const DB_NAME: &str = "connect2discord";
const DB_USRNAME: &str = "admin";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Settings {
    embedColor: String
}

#[derive(Serialize, Deserialize, CouchDocument, Default, Debug)]
pub struct GuildEntry {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,

    pub guildId: String,
    pub settings: Settings
}

#[get("/db/get/{id}")]
pub async fn db_get_settings(path: Path<String>) -> Result<HttpResponse, Box<dyn Error>> {
    let id = path.into_inner();
    
    let client = couch_rs::Client::new(DB_HOST, DB_USRNAME, &std::env::var("CDB_PASS")?)?;
    let db = client.db(DB_NAME).await?;
    let query = FindQuery::new(json!({
        "guildId": id
    }));
    let docs: DocumentCollection<GuildEntry> = db.find(&query).await?;

    let json = &docs.get_data()[0];
    let mut response = HttpResponse::Ok();
    response.append_header(("Content-Type", "application/json"));
    Ok(response.body(json2string(&json.settings)?))
} 
