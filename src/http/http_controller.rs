use rocket::{get, post};
use rocket::response::status;
use rocket::serde::json::Json;
use serde_json::Value;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;

use crate::chat::bot::deza;
use crate::chat::db_mongo::MongoDb;
use crate::cofg::JobWP;
use crate::http::models::FacebookToken;


#[post("/whatsapp/bot/insert", format = "application/json", data = "<task>")]
pub async fn insert(db:MongoDb<'_>,task: Json<Value>) -> Result<status::Created<String>,status::BadRequest<String>> {

    match  db.set_bot(task.0.clone()).await {
        Ok(c) => {

            deza(&task.0,&db).await;

            Ok(status::Created::new("").body(c.to_string()))},
        Err(e) => Err(status::BadRequest(Some(e)))
    }
}

#[post("/facebook/token", format = "application/json", data = "<task>")]
pub async fn facebook_token(db:MongoDb<'_>,task: Json<FacebookToken>) -> Result<status::Created<String>,status::BadRequest<String>> {

    match  db.insert_token_facebook(&task.0).await {
        Ok(c) => {
            Ok(status::Created::new("").body(c.to_string()))},
        Err(e) => Err(status::BadRequest(Some(e)))
    }
}


#[get("/whatsapp/bot/get")]
pub async fn get(db:MongoDb<'_>) -> Result<status::Accepted<Json<Vec<Value>>>,status::BadRequest<String>> {

    let f =db.get_bot().await.unwrap();



       Ok(status::Accepted(Some(Json(f))))

    }

