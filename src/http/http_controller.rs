use std::io::BufRead;
use reqwest::{Client, Error, Response};
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
pub async fn insert(db: MongoDb<'_>, task: Json<Value>) -> Result<status::Created<String>, status::BadRequest<String>> {
    match db.set_bot(task.0.clone()).await {
        Ok(c) => {
            deza(&task.0, &db).await;

            Ok(status::Created::new("").body(c.to_string()))
        }
        Err(e) => Err(status::BadRequest(Some(e)))
    }
}

#[post("/facebook/token", format = "application/json", data = "<task>")]
pub async fn facebook_token(db: MongoDb<'_>, task: Json<FacebookToken>) -> Result<status::Created<String>, status::BadRequest<String>> {
    let req: Client = Client::new();

    let mut face: FacebookToken = task.0;

  let id =  match req.get(format!("https://graph.facebook.com/v16.0/me/accounts?access_token={}", face.access_token.clone()))
        .send().await {
        Ok(x) => {
            let value :serde_json::Value =   x.json().await.unwrap();
           let data = value.get("data").unwrap();
            println!("{:?}",data);
           let arr = &data.as_array().unwrap()[0];
            let id   = arr.get("id").unwrap().to_string();
           Ok( id )
        }
        Err(e) => { Err(e.to_string()) }
    };


    match id {
        Ok(c) => {
            face.page = Some(c);
            match db.insert_token_facebook(&face).await {
                Ok(c) => {
                    Ok(status::Created::new("").body(c.to_string()))
                }
                Err(e) => Err(status::BadRequest(Some(e)))
            }

        }
        Err(e) => Err(status::BadRequest(Some(e)))
    }


}


#[get("/whatsapp/bot/get")]
pub async fn get(db: MongoDb<'_>) -> Result<status::Accepted<Json<Vec<Value>>>, status::BadRequest<String>> {
    let f = db.get_bot().await.unwrap();
    Ok(status::Accepted(Some(Json(f))))
}

