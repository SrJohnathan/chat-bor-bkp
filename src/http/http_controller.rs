use rocket::{get, post};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde_json::Value;
use crate::chat::bot::deza;
use crate::chat::db_mongo::MongoDb;



#[post("/whatsapp/bot/insert", format = "application/json", data = "<task>")]
pub async fn insert(db:MongoDb<'_>,task: Json<Value>) -> Result<status::Created<String>,status::BadRequest<String>> {
    match  db.set_bot(task.0.clone()).await {
        Ok(c) => {

            deza(&task.0,&db).await;

            Ok(status::Created::new("").body(c.to_string()))},
        Err(e) => Err(status::BadRequest(Some(e)))
    }
}

#[get("/whatsapp/bot/get")]
pub async fn get(db:MongoDb<'_>) -> Result<status::Accepted<Json<Vec<Value>>>,status::BadRequest<String>> {

    let f =db.get_bot().await.unwrap();



       Ok(status::Accepted(Some(Json(f))))

    }

