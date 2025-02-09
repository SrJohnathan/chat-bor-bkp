use std::future::Future;
use reqwest::{Client, Error, Response};
use rocket::http::Status;
use rocket::response::status::{BadRequest, Created};
use rocket::serde::json::Json;
use rocket::{post, State};
use serde_json::to_string;
use tokio::sync::mpsc::Sender;
use crate::chat::{ChatWP};
use crate::chat::db_mongo::MongoDb;
use crate::chat::send_list_wp::{MessageText, SendWP};
use crate::cofg::{get_number_app, NewJob};
use crate::http::models::{Audio, ButtonReply, Delivered, Enqueued, Failed, File, Image, ListReply, Location, MessageEvent, MessageGP, ParentMessage, Read, Sent, Text, Video};
use crate::model::mongo::{select_status};

#[post("/whatsapp/chatbot", format = "application/json", data = "<task>")]
pub async fn web_hook(db: MongoDb<'_>, job: &State<Sender<String>>, task: Json<serde_json::Value>)
                      -> Status {
    let message = task.0;
    let d = message.get("type");
    let req: Client = Client::new();


    println!("{}",message);

    match d {
        None => { println!("não encontrou type") }
        Some(c) => {

            // println!("{:?}",message);

            let app = message.get("app").unwrap();


            if c.as_str().unwrap().eq("message-event") {
                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();


                if ty.as_str().unwrap().eq(&"enqueued".to_string()) {
                    let msg: ParentMessage<MessageEvent<Enqueued>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"failed".to_string()) {
                    let msg: ParentMessage<MessageEvent<Failed>> = serde_json::from_str(&message.to_string()).unwrap();

                    println!("{:?}", msg);
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
                let mut chat = ChatWP::new(number.as_str().unwrap(), app.as_str().unwrap());


                if ty.as_str().unwrap().eq(&"text".to_string()) {


                    let msg: ParentMessage<MessageGP<Text>> = serde_json::from_str(&message.to_string()).unwrap();


                    let name = req.post("https://siga-telecom.herokuapp.com/api/v1/whatsapp/webHookSocket")
                        // .header("Content-Type", "application/json")
                        .json(&msg)
                        .send().await;


                    match name {
                        Ok(x) => {
                            let tex = x.text().await.unwrap();
                            if tex != "null".to_string() {
                                chat.add_props(String::from("nodedouser"), tex);
                            } else {
                                chat.add_props(String::from("nodedouser"), msg.payload.sender.name);
                            }
                        }
                        Err(e) => {
                            chat.add_props(String::from("nodedouser"), msg.payload.sender.name);
                        }
                    }



                        match chat.run(&db).await {
                            Ok(c) => {
                                let e = NewJob {
                                    number: c.number.clone(),
                                    etapa: c.st.clone(),
                                    time: 0,
                                    app: c.app.clone(),
                                };

                                match job.send(serde_json::to_string(&e).unwrap()).await {
                                    Ok(x) => {}
                                    Err(e) => { println!("{}", e.0) }
                                }
                            }
                            Err(e) => { println!("erro {}", e) }
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


                    req.post("https://siga-telecom.herokuapp.com/api/v1/whatsapp/webHookSocket")
                        // .header("Content-Type", "application/json")
                        .json(&msg)
                        .send().await;


                    match chat.run_button(&msg.payload.payload.title, &db).await {
                        Ok(c) => {
                            let e = if c.st.as_str() == "exit" {
                                NewJob {
                                    number: c.number.clone(),
                                    etapa: "exit".to_string(),
                                    time: 0,
                                    app: c.app.clone(),
                                }
                            } else {
                                NewJob {
                                    number: c.number.clone(),
                                    etapa: c.st.clone(),
                                    time: 0,
                                    app: c.app.clone(),
                                }
                            };

                            match job.send(serde_json::to_string(&e).unwrap()).await {
                                Ok(x) => {}
                                Err(e) => { println!("{}", e.0) }
                            }
                        }
                        Err(e) => { println!("erro {}", e) }
                    }


                } else if ty.as_str().unwrap().eq(&"list_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ListReply>> = serde_json::from_str(&message.to_string()).unwrap();


                    req.post("https://siga-telecom.herokuapp.com/api/v1/whatsapp/webHookSocket")
                        // .header("Content-Type", "application/json")
                        .json(&msg)
                        .send().await;

                    chat.add_props(String::from("nodedouser"), msg.payload.sender.name);

                    let tmpstr = msg.payload.payload.postbackText.replace("n", "");
                    let my_str = tmpstr.trim().parse::<i32>().unwrap();


                    match msg.payload.payload.title.as_str() {
                        "Voltar" => {
                            match chat.back(&db).await {
                                Ok(c) => {}
                                Err(e) => {}
                            }
                        }

                        &_ => {
                            match chat.run_list(&(my_str + 1).to_string(), &db).await {
                                Ok(c) => {
                                    let e = NewJob {
                                        number: c.number.clone(),
                                        etapa: c.st.clone(),
                                        time: 0,
                                        app: c.app.clone(),
                                    };

                                    match job.send(serde_json::to_string(&e).unwrap()).await {
                                        Ok(x) => {}
                                        Err(e) => { println!("{}", e.0) }
                                    }
                                }
                                Err(e) => { println!("erro {}", e) }
                            }
                        }
                    }
                } else {}
            } else {}
        }
    }


    Status::Ok
}


