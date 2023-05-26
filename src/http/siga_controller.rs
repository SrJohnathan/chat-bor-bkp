use reqwest::{Client, Error, Response};
use rocket::response::status;
use rocket::serde::json::Json;
use crate::chat::db_mongo::MongoDb;
use rocket::{get, post};
use rocket::http::Status;
use rocket::response::status::Created;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task;
use crate::http::models::{Audio, ButtonReply, Delivered, Enqueued, Failed, File, Image, ListReply, Location, MessageEvent, MessageGP, ParentMessage, Read, Sent, Text, Video};
use crate::{get_number_app, MessageText, SendMessage, SendWP};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ReadWT {
    pub r#type: String,
    pub text: String,
    pub app: String,
    pub number: String,
}

#[post("/agent/send", format = "application/json", data = "<task>")]
pub async fn send(db: MongoDb<'_>, task: Json<ReadWT>) -> Result<Created<String>, String> {
    let key = std::env::var("KEY_API").unwrap();
    let req: Client = Client::new();
    let wt = task.0;

    let result = serde_json::to_value(
        MessageText { type_field: "text".to_string(), text: wt.text }
    ).unwrap();

    let value: SendWP<Value> = SendWP::new(
        wt.app.as_str(),
        wt.number.as_str(), get_number_app(wt.app.as_str()),
        result);

    let send = SendMessage::new(key.clone());

     let respo = send.sendNoTime(&value).await;

    match respo {
        Ok(e) => {
            Ok(status::Created::new("".to_string()).body(e))
        }
        Err(s) => { Err(s.to_string()) }
    }
}



#[post("/agent/receiver", format = "application/json", data = "<task>")]
pub async fn agente(task: Json<serde_json::Value>) -> Result<Created<String>, String> {
    let message = task.0;
    let d = message.get("type");
    let req: Client = Client::new();

    match d {
        None => { Ok(status::Created::new("".to_string()).body("".to_string())) }
        Some(c) => {
            let app = message.get("app").unwrap();


            if c.as_str().unwrap().eq("message-event") {
                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();

                if ty.as_str().unwrap().eq(&"enqueued".to_string()) {
                    let msg: ParentMessage<MessageEvent<Enqueued>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"failed".to_string()) {
                    let msg: ParentMessage<MessageEvent<Failed>> = serde_json::from_str(&message.to_string()).unwrap();

                    println!("{:?}",msg);

                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"sent".to_string()) {
                    let msg: ParentMessage<MessageEvent<Sent>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"delivered".to_string()) {
                    let msg: ParentMessage<MessageEvent<Delivered>> = serde_json::from_str(&message.to_string()).unwrap();

                   /* tokio::spawn(async move {
                        let response = req.post("https://siga-telecom.herokuapp.com/api/v1/whatsapp/webHookSocket")
                            // .header("Content-Type", "application/json")
                            .json(&msg)
                            .send().await;
                        match response {
                            Ok(e) => { Ok(status::Created::new("".to_string()).body("".to_string())) }
                            Err(s) => { Err(s.to_string()) }
                        }
                    }); */


                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"read".to_string()) {
                    let msg: ParentMessage<MessageEvent<Read>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else {
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                }
            } else if c.as_str().unwrap().eq("message") {
                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();

                if ty.as_str().unwrap().eq(&"text".to_string()) {
                    let msg: ParentMessage<MessageGP<Text>> = serde_json::from_str(&message.to_string()).unwrap();


                    tokio::spawn(async move {
                        let response = req.post("https://siga-telecom.herokuapp.com/api/v1/whatsapp/webHookSocket")
                            // .header("Content-Type", "application/json")
                            .json(&msg)
                            .send().await;
                        match response {
                            Ok(e) => { Ok(status::Created::new("".to_string()).body("".to_string())) }
                            Err(s) => { Err(s.to_string()) }
                        }
                    });

                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"image".to_string()) {
                    let msg: ParentMessage<MessageGP<Image>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"file".to_string()) {
                    let msg: ParentMessage<MessageGP<File>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"audio".to_string()) {
                    let msg: ParentMessage<MessageGP<Audio>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"video".to_string()) {
                    let msg: ParentMessage<MessageGP<Video>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"location".to_string()) {
                    let msg: ParentMessage<MessageGP<Location>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"button_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ButtonReply>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"list_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ListReply>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else {
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                }
            } else {
                Ok(status::Created::new("".to_string()).body("".to_string()))
            }
        }
    }
}
