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

    let id = match req.get(format!("https://graph.facebook.com/v16.0/me/accounts?access_token={}", face.access_token.clone()))
        .send().await {
        Ok(x) => {
            let value : serde_json::Value = x.json().await.unwrap();
            if let Some(data) = value.get("data") {
                if let Some(arr) = data.as_array().and_then(|arr| arr.get(0)) {
                    let id = match arr.get("id") {
                        Some(id) => Ok(id.as_str().unwrap().to_string()),
                        None => Err("id field not found".to_string())
                    };
                    id
                } else {
                    Err("empty array".to_string())
                }
            } else {
                Err("data field not found".to_string())
            }
        }
        Err(e) => Err(e.to_string())
    };


    match id {
        Ok(c) => {
            face.page = Some(String::from(c));
            let page_token = page_token(face.page.clone().unwrap(),face.long_lived_token.clone(),req).await;

            match  page_token {
                Ok(x) => {

                    face.page_token = Some(String::from(x.1));
                    match db.insert_or_update_facebook_token(&face).await {
                        Ok(c) => {
                            Ok(status::Created::new("").body(c.to_string()))
                        }
                        Err(e) => Err(status::BadRequest(Some(e)))
                    }


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

async fn page_token(page_id:String, acess_token:String, req: Client ) -> Result<(String, String), String> {
     match req.get(format!("https://graph.facebook.com/{}?fields=access_token&access_token={}",page_id, acess_token))
        .send().await{
        Ok(x) => {
            let mut value : serde_json::Value = x.json().await.unwrap();

            let ac = value.get("access_token").unwrap().as_str().unwrap();
            let id = value.get("id").unwrap().as_str().unwrap();

           Ok( (  id.to_string(),ac.to_string() ) )
        }
        Err(x) =>  Err(x.to_string())
    }



}