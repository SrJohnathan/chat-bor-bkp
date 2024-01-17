use reqwest::{Client, Error, Response, StatusCode};
use rocket::response::status;
use rocket::serde::json::Json;
use crate::chat::db_mongo::MongoDb;
use rocket::{get, post, put};
use rocket::http::Status;
use rocket::response::status::Created;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task;
use crate::http::models::{Audio, ButtonReply, Delivered, Enqueued, Failed, File, Image, ListReply, Location, MessageEvent, MessageGP, ParentMessage, Read, Sent, Text, Video};
use crate::{get_number_app, MessageText, SendMessage, SendWP};
use crate::chat::ChatWP;
use crate::chat::send_list_wp::{ImageMidia, MidiaType, TemplateText};
use crate::cofg::{api_leads, get_app_app, get_app_id, HOST_API_GUPSHUP, Leads, NewJob};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ReadWT {
    pub r#type: String,
    pub text: String,
    pub app: String,
    pub number: String,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ReadMessage {
    pub app: String,
    pub idm: String,

}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ReadTemplate {
    pub r#type: String,
    pub id: String,
    pub params: Vec<String>,
    pub app: String,
    pub number: String,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ReadWTDoc {
    pub r#type: String,
    pub payload: Docc,
    pub app: String,
    pub number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Docc {
    #[serde(rename = "type")]
    pub type_field: String,
    pub original_url: String,
    pub preview_url: String,
    pub caption: Option<String>,
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


#[put("/system/read", format = "application/json", data = "<task>")]
pub async fn read_system(task: Json<ReadMessage>) -> Result<Status,String> {

    let req: Client = Client::new();
    let wt = task.0;

let url = format!("{}/wa/app/{}/msg/{}/read", HOST_API_GUPSHUP, get_app_id( wt.app.as_str()) ,wt.idm );
    let response = req.put(url)
        .header("apikey", get_app_app(wt.app.as_str()))
        // .header("Content-Length", content_length.to_string())
        .send().await;

      match  response {
          Ok(x) => {  Ok( Status::new(x.status().as_u16() )) }
          Err(x) => {  Err(x.to_string()) }
      }
}


#[get("/template/<appName>")]
pub async fn read_template(appName:String) -> Result<Created<String>, String> {

    let req: Client = Client::new();
    let url = format!("{}/sm/api/v1/template/list/{}", HOST_API_GUPSHUP, appName );
    let response = req.get(url)
        .header("apikey", get_app_app(appName.as_str()))
        // .header("Content-Length", content_length.to_string())
        .send().await;
    match response {
        Ok(e) => {
            Ok(status::Created::new("".to_string()).body(  e.text().await.unwrap()  ))
        }
        Err(s) => { Err(s.to_string()) }
    }
}



#[get("/money")]
pub async fn money() -> Result<Created<String>, String> {

    let req: Client = Client::new();

    let response = req.get("https://api.gupshup.io/sm/api/v2/wallet/balance")
        .header("apikey", "ku8gzeihiztucp71pog5xoipestl5abp")
        // .header("Content-Length", content_length.to_string())
        .send().await;
    match response {
        Ok(e) => {
            Ok(status::Created::new("".to_string()).body(  e.text().await.unwrap()  ))
        }
        Err(s) => { Err(s.to_string()) }
    }
}




#[post("/agent/template", format = "application/json", data = "<task>")]
pub async fn template(db: MongoDb<'_>, task: Json<ReadTemplate>) -> Result<Created<String>, String> {
    let key = std::env::var("KEY_API").unwrap();
    let req: Client = Client::new();
    let wt = task.0;

    let result = serde_json::to_value(
        TemplateText { id: wt.id , params: wt.params }
    ).unwrap();

    let value: SendWP<Value> = SendWP::new(
        wt.app.as_str(),
        wt.number.as_str(), get_number_app(wt.app.as_str()),
        result);

    let send = SendMessage::new(key.clone());

    let respo = send.sendTemplate(&value).await;

    match respo {
        Ok(e) => {
            Ok(status::Created::new("".to_string()).body(e))
        }
        Err(s) => { Err(s.to_string()) }
    }
}


#[post("/agent/sendArchive", format = "application/json", data = "<task>")]
pub async fn send_archive(db: MongoDb<'_>, task: Json<ReadWTDoc>) -> Result<Created<String>, String> {
    let key = std::env::var("KEY_API").unwrap();
    let req: Client = Client::new();
    let wt = task.0;

    println!("{:?}", wt);

    let result = if wt.r#type == "image".to_string() {
        serde_json::to_value(
            ImageMidia {
                type_field: "image".to_string(),
                caption: match   wt.payload.caption {
                    None => "".to_string(),
                    Some(x) => x
                },
                original_url: wt.payload.original_url,
                preview_url: wt.payload.preview_url,
            }
        ).unwrap()
    } else if wt.r#type == "file".to_string() {
        serde_json::to_value(
            MidiaType {
                type_field: "file".to_string(),
                url: wt.payload.original_url,
                filename: wt.payload.caption,

            }
        ).unwrap()

    } else if wt.r#type == "audio/mpeg".to_string() {
        serde_json::to_value(
            MidiaType {
                type_field: "audio".to_string(),
                url: wt.payload.original_url,
                filename: None

            }
        ).unwrap()

    }
    else {
        serde_json::to_value(
            MessageText { type_field: "text".to_string(), text: "NOT_FOUND".to_string() }
        ).unwrap()
    };

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
pub async fn agente(db: MongoDb<'_>,task: Json<serde_json::Value>) -> Result<Created<String>, String> {
    let message: serde_json::Value = task.0;
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
                } else {
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                }
            } else if c.as_str().unwrap().eq("message") {

                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();



                if ty.as_str().unwrap().eq(&"text".to_string()) {

                    let msg: ParentMessage<MessageGP<Text>> = serde_json::from_str(&message.to_string()).unwrap();
                    

                     let res =  api_leads(&msg.payload.source).await;

                    
                        match res {
                            Ok(x) => {

                                tokio::spawn(async move {
                                    let response = req.post("https://apibotstw-ecd4d17d82c8.herokuapp.com/receiver")
                                        // .header("Content-Type", "application/json")
                                        .json(&msg)
                                        .send().await;
                                    match response {
                                        Ok(e) => { Ok(status::Created::new("".to_string()).body("".to_string())) }
                                        Err(s) => { Err(s.to_string()) }
                                    }
                                });


                            }
                            Err(e) => {


                                println!("{}",app.as_str().unwrap());

                                let mut chat = ChatWP::new(msg.payload.source.as_str(), app.as_str().unwrap());
                                chat.add_props(String::from("nodedouser"), msg.payload.sender.name);

                                match chat.run(&db).await {
                                    Ok(c) => {

                                        println!("{:?}",c)

                                       /* match job.send(serde_json::to_string(&e).unwrap()).await {
                                            Ok(x) => {}
                                            Err(e) => { println!("{}", e.0) }
                                        }*/
                                    }
                                    Err(e) => { println!("erro {}", e) }
                                }

                            }
                        };

                   


                    Ok(status::Created::new("".to_string()).body("".to_string()))


                } else if ty.as_str().unwrap().eq(&"image".to_string()) {
                    let msg: ParentMessage<MessageGP<Image>> = serde_json::from_str(&message.to_string()).unwrap();


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



                } else if ty.as_str().unwrap().eq(&"file".to_string()) {


                    let msg: ParentMessage<MessageGP<File>> = serde_json::from_str(&message.to_string()).unwrap();

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
                } else if ty.as_str().unwrap().eq(&"audio".to_string()) {
                    let msg: ParentMessage<MessageGP<Audio>> = serde_json::from_str(&message.to_string()).unwrap();

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
                } else if ty.as_str().unwrap().eq(&"video".to_string()) {
                    let msg: ParentMessage<MessageGP<Video>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"location".to_string()) {
                    let msg: ParentMessage<MessageGP<Location>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"button_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ButtonReply>> = serde_json::from_str(&message.to_string()).unwrap();


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
                } else if ty.as_str().unwrap().eq(&"quick_reply".to_string()) {




                    let msg: ParentMessage<MessageGP<Text>> = serde_json::from_str(&message.to_string()).unwrap();

                    tokio::spawn(async move {
                        let response = req.post("https://siga-telecom.herokuapp.com/api/v1/whatsapp/webHookSocket")
                            // .header("Content-Type", "application/json")
                            .json(&msg)
                            .send().await;
                        match response {
                            Ok(e) => {



                                Ok(status::Created::new("".to_string()).body("".to_string())) }
                            Err(s) => { Err(s.to_string()) }
                        }
                    });

                    Ok(status::Created::new("".to_string()).body("".to_string()))



                } else

                if ty.as_str().unwrap().eq(&"list_reply".to_string()) {
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


/*#[post("/agent/countdown", format = "application/json", data = "<task>")]
pub async fn count(db: MongoDb<'_>, task: Json<ReadWT>) -> Result<Created<String>, String> {



}*/
