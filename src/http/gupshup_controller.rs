use rocket::http::Status;
use rocket::response::status::{BadRequest, Created};
use rocket::serde::json::Json;
use rocket::{post};
use crate::chat::{ ChatWP};
use crate::chat::db_mongo::MongoDb;
use crate::http::models::{Audio, ButtonReply, Delivered, Enqueued, Failed, File, Image, ListReply, Location, MessageEvent, MessageGP, ParentMessage, Read, Sent, Text, Video};
use crate::model::mongo::{ select_status};

#[post("/whatsapp/chatbot", format = "application/json", data = "<task>")]
pub async fn web_hook(db:MongoDb<'_>,task: Json<serde_json::Value>)
    -> Status {

    let message = task.0;
    let d = message.get("type");



    match d {
        None => { println!("não encontrou type") }
        Some(c) => {

            println!("{:?}",message);

            let app = message.get("app").unwrap();


            if c.as_str().unwrap().eq("message-event") {
                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();



                if ty.as_str().unwrap().eq(&"enqueued".to_string()) {
                    let msg: ParentMessage<MessageEvent<Enqueued>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"failed".to_string()) {
                    let msg: ParentMessage<MessageEvent<Failed>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"sent".to_string()) {
                    let msg: ParentMessage<MessageEvent<Sent>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"delivered".to_string()) {
                    let msg: ParentMessage<MessageEvent<Delivered>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"read".to_string()) {
                    let msg: ParentMessage<MessageEvent<Read>> = serde_json::from_str(&message.to_string()).unwrap();
                } else {}
            } else if c.as_str().unwrap().eq("message") {

                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();
                let number = pl.get("source").unwrap();
                let chat = ChatWP::new(number.as_str().unwrap(),app.as_str().unwrap());






                if ty.as_str().unwrap().eq(&"text".to_string()) {

                    let msg: ParentMessage<MessageGP<Text>> = serde_json::from_str(&message.to_string()).unwrap();


                    match  chat.run(&db).await {
                        Ok(c) => {println!("{}",c) }
                        Err(e) => { println!("erro {}",e)}
                    }

                } else if ty.as_str().unwrap().eq(&"image".to_string()) {
                    let msg: ParentMessage<MessageGP<Image>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"file".to_string()) {
                    let msg: ParentMessage<MessageGP<File>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"audio".to_string()) {
                    let msg: ParentMessage<MessageGP<Audio>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"video".to_string()) {
                    let msg: ParentMessage<MessageGP<Video>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"location".to_string()) {
                    let msg: ParentMessage<MessageGP<Location>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"button_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ButtonReply>> = serde_json::from_str(&message.to_string()).unwrap();


                    println!("{:?}",msg.payload.payload.title);

                    match  chat.run_button(&msg.payload.payload.title,&db).await {
                        Ok(c) => {println!("{}",c) }
                        Err(e) => { println!("erro {}",e)}
                    }


                } else if ty.as_str().unwrap().eq(&"list_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ListReply>> = serde_json::from_str(&message.to_string()).unwrap();

                    let my_str = msg.payload.payload.title.trim();
                    let digits = my_str.chars().last().unwrap().to_digit(10).unwrap();

                    match  chat.run_list(&digits.to_string(),&db).await {
                        Ok(c) => {println!("{}",c) }
                        Err(e) => { println!("erro {}",e)}
                    }

                } else {}



            }else {}
        }
    }


    Status::Ok


}