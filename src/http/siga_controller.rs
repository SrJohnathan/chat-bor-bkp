use chrono::{DateTime, Utc};
use chrono_tz::Europe::Lisbon;
use reqwest::{Client, };
use rocket::response::status;
use rocket::serde::json::Json;
use crate::chat::db_mongo::MongoDb;
use rocket::{get, post, put, State};

use rocket::http::Status;
use rocket::response::status::{Accepted, BadRequest, Created};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::Sender;

use crate::http::models::{Audio, BotClient, ButtonReply, Delivered, Enqueued, Failed, File, Image, ListReply, Location, MessageEvent, MessageGP, ParentMessage, Read, SendData, Sent, Text, Video};
use crate::{get_number_app, MessageText, SendMessage, SendWP};
use crate::chat::ChatWP;
use crate::chat::send_list_wp::{ImageMidia, MidiaType, TemplateText};
use crate::chat::structs::{Chat, ChatDataType, ClientKeyBot};
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
pub async fn read_system(task: Json<ReadMessage>) -> Result<Status, String> {
    let req: Client = Client::new();
    let wt = task.0;

    let url = format!("{}/wa/app/{}/msg/{}/read", HOST_API_GUPSHUP, get_app_id(wt.app.as_str()), wt.idm);
    let response = req.put(url)
        .header("apikey", get_app_app(wt.app.as_str()))
        // .header("Content-Length", content_length.to_string())
        .send().await;

    match response {
        Ok(x) => { Ok(Status::new(x.status().as_u16())) }
        Err(x) => { Err(x.to_string()) }
    }
}


#[get("/template/<appName>")]
pub async fn read_template(appName: String) -> Result<Created<String>, String> {
    let req: Client = Client::new();
    let url = format!("{}/sm/api/v1/template/list/{}", HOST_API_GUPSHUP, appName);
    let response = req.get(url)
        .header("apikey", get_app_app(appName.as_str()))
        // .header("Content-Length", content_length.to_string())
        .send().await;
    match response {
        Ok(e) => {
            Ok(status::Created::new("".to_string()).body(e.text().await.unwrap()))
        }
        Err(s) => { Err(s.to_string()) }
    }
}

#[get("/getBots/<appName>")]
pub async fn get_clients_bots(db: MongoDb<'_>, appName: String) -> Result<Accepted<Json<Vec<BotClient>>>, rocket::response::status::BadRequest<String>> {
   match    db.get_all_client_key_bots_by_app(&appName).await {
       Ok(x) => Ok(  Accepted(Some(Json(x))) ),
       Err(x) => Err( status::BadRequest(Some(x.to_string())) )
   }
}


#[get("/getChat/<appName>/<state>")]
pub async fn get_clients_chat(db: MongoDb<'_>, appName: String, state:String) -> Result<Accepted<Json<Vec<SendData<Value>>>>, BadRequest<String>> {


    return  match    db.get_all_client_bot(&state).await {
        Ok(v) => Ok(status::Accepted(Some(Json(v)))),
        Err(x) => Err( status::BadRequest(Some(x.to_string())) )
    }
}



#[put("/updateBots/<number>/<boolean>")]
pub async fn updateBots(db: MongoDb<'_>, number: String,boolean:bool)  -> Result<rocket::response::status::Accepted<()>, std::string::String>  {



    match  db.update_show_field(number,boolean).await {
        Ok(x) =>  Ok(status::Accepted::<()>(None)),
        Err(x) =>  Err(x.to_string())
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
            Ok(status::Created::new("".to_string()).body(e.text().await.unwrap()))
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
        TemplateText { id: wt.id, params: wt.params }
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
                caption: match wt.payload.caption {
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
                filename: None,
            }
        ).unwrap()
    } else {
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
pub async fn agente(db: MongoDb<'_>, job: &State<Sender<String>>, task: Json<serde_json::Value>) -> Result<Created<String>, String> {
    let message: Value = task.0;
    let d = message.get("type");
    let req: Client = Client::new();

    let utc: DateTime<Utc> = Utc::now();
    let lisbon_time= utc.with_timezone(&Lisbon);

    println!("{}", message);


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
                        let response = req.post("https://apibotstw-ecd4d17d82c8.herokuapp.com/receiver")
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
                } else {
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                }
            } else if c.as_str().unwrap().eq("message") {
                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();


                if ty.as_str().unwrap().eq(&"text".to_string()) {
                    let msg: ParentMessage<MessageGP<Text>> = serde_json::from_str(&message.to_string()).unwrap();


                    let res = api_leads(&msg.payload.source).await;
                    match res {
                        Ok(x) => {
                            tokio::spawn(async move {
                                let response = req.post("https://apibotstw-ecd4d17d82c8.herokuapp.com/receiver")
                                    // .header("Content-Type", "application/json")
                                    .json(&msg)
                                    .send().await;
                                match response {
                                    Ok(e) => {
                                        Ok(status::Created::new("".to_string()).body("".to_string())) }
                                    Err(s) => { Err(s.to_string()) }
                                }
                            });
                        }
                        Err(e) => {
                            let mut chat = ChatWP::new(msg.payload.source.as_str(), app.as_str().unwrap());
                            chat.add_props(String::from("nodedouser"), msg.payload.sender.name.clone());


                        match db.get_status(&msg.payload.source.to_string(), &app.as_str().unwrap().to_string()).await {
                            Ok(x) => {

                              if   x.st == "1" {
                                    match   db.update_session(format!("+{}" ,msg.payload.source.clone()) ).await {
                                        Ok(x) => {

                                            if x == true {
                                                chat.add_props("voltar".to_string(), "true".to_string());
                                            }

                                        }
                                        Err(e) => {   }
                                    };
                                }




                            }
                            Err(e) => {

                                match   db.update_session( format!("+{}" ,msg.payload.source.clone())  ).await {
                                    Ok(x) => {

                                        if x == true {
                                            chat.add_props("voltar".to_string(), "true".to_string());
                                        }

                                    }
                                    Err(e) => {   }
                                };

                            }
                        };





                            match chat.run(&db).await {
                                Ok(c) => {

                                    let  data : SendData<Value>  = SendData{
                                        data :message,
                                        position: 0,
                                        show:true,
                                        type_field:1,
                                        sid: format!("+{}",msg.payload.source),
                                        time: lisbon_time.naive_utc().to_string(),
                                        id:None,
                                        name: msg.payload.sender.name,
                                        id_user:None
                                    };
                                    db.set_key_client(data).await.unwrap();



                                    let e = NewJob {
                                        number: c.number.clone(),
                                        etapa: c.st.clone(),
                                        time: 0,
                                        app: c.app.clone(),
                                    };

                                    match job.send(serde_json::to_string(&e).unwrap()).await {
                                        Ok(x) => {




                                        }
                                        Err(e) => { println!("{}", e.0) }
                                    }
                                }
                                Err(e) => { println!("erro {}", e) }
                            }
                        }
                    };


                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"image".to_string()) {
                    let msg: ParentMessage<MessageGP<Image>> = serde_json::from_str(&message.to_string()).unwrap();


                    let res = api_leads(&msg.payload.source).await;
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
                        Err(e) => {}
                    };


                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"file".to_string()) {
                    let msg: ParentMessage<MessageGP<File>> = serde_json::from_str(&message.to_string()).unwrap();

                    let res = api_leads(&msg.payload.source).await;
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
                        Err(e) => {}
                    };


                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"audio".to_string()) {
                    let msg: ParentMessage<MessageGP<Audio>> = serde_json::from_str(&message.to_string()).unwrap();

                    let res = api_leads(&msg.payload.source).await;
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
                        Err(e) => {}
                    };



                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"video".to_string()) {
                    let msg: ParentMessage<MessageGP<Video>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"location".to_string()) {
                    let msg: ParentMessage<MessageGP<Location>> = serde_json::from_str(&message.to_string()).unwrap();
                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"button_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ButtonReply>> = serde_json::from_str(&message.to_string()).unwrap();


                    let res = api_leads(&msg.payload.source).await;
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
                            let mut chat = ChatWP::new(msg.payload.source.as_str(), app.as_str().unwrap());
                            chat.add_props(String::from("nodedouser"), msg.payload.sender.name.clone());





                            match chat.run_button(&msg.payload.payload.title, &db).await {
                                Ok(c) => {


                                    let  data : SendData<Value>  = SendData{
                                        data :message,
                                        position: 0,
                                        show:true,
                                        type_field:1,
                                        sid: format!("+{}",msg.payload.source),
                                        time: lisbon_time.naive_utc().to_string(),
                                        id:None,
                                        name: msg.payload.sender.name,
                                        id_user:None
                                    };
                                    db.set_key_client(data).await.unwrap();



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
                        }
                    };


                    Ok(status::Created::new("".to_string()).body("".to_string()))
                } else if ty.as_str().unwrap().eq(&"quick_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<Text>> = serde_json::from_str(&message.to_string()).unwrap();


                    let res = api_leads(&msg.payload.source).await;
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
                        Err(e) => {}
                    };


                    Ok(status::Created::new("".to_string()).body("".to_string()))



                } else

                if ty.as_str().unwrap().eq(&"list_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ListReply>> = serde_json::from_str(&message.to_string()).unwrap();


                    let res = api_leads(&msg.payload.source).await;
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
                            let mut chat = ChatWP::new(msg.payload.source.as_str(), app.as_str().unwrap());
                            chat.add_props(String::from("nodedouser"), msg.payload.sender.name.clone());






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


                                            let  data : SendData<Value>  = SendData{
                                                data :message,
                                                position: 0,
                                                show:true,
                                                type_field:1,
                                                sid: format!("+{}",msg.payload.source),
                                                time: lisbon_time.naive_utc().to_string(),
                                                id:None,
                                                name: msg.payload.sender.name,
                                                id_user:None
                                            };
                                            db.set_key_client(data).await.unwrap();






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
                        }
                    };


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


