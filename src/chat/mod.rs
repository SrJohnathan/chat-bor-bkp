use std::collections::HashMap;
use std::fmt::format;
use std::num::ParseIntError;
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Lisbon;
use mongodb::error::Error;
use rocket::response::status;
use serde_json::Value;
use crate::chat::db_mongo::MongoDb;
use crate::chat::send_list_wp::{MessageText, SendWP};
use crate::chat::structs::ClientKeyBot;
use crate::chat::structs::status::Status;
use crate::cofg::get_number_app;
use crate::http::models::SendMessage;
use crate::model::mongo::{insert_status, select_status, update_status};


pub mod bot;
pub mod send_list_wp;
pub mod structs;
pub mod db_mongo;
pub mod factory_msg_send_text;
pub mod models_instagram;


#[derive(Clone)]
pub struct ChatWP {
    number: String,
    app: String,
    pub map: HashMap<String, String>,
}


impl ChatWP {
    pub fn new(number: &str, app: &str) -> Self {
        ChatWP { number: number.to_string(), app: app.to_string(), map: HashMap::new() }
    }

    pub fn add_props(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub async fn run( &mut self, con: &MongoDb<'_>) -> Result<Status, String> {
        let res = select_status(self.number.clone(), self.app.clone(), con.0).await;
        let key = std::env::var("KEY_API").unwrap();


        match res {
            Ok(c) => {

                if c.len() > 0 {


                    let st: &Status = c.get(0).unwrap();

                    if st.st == "1".to_string() {
                       
                    }





                    match   status_not_key(st.st.clone()) {
                        Ok(x) => {

                            let result = serde_json::to_value(
                                MessageText { type_field: "text".to_string(), text: "Escolha uma opção por favor".to_string()}
                            ).unwrap();

                            let value: SendWP<Value> = SendWP::new(
                                st.app.as_str(),
                                st.number.as_str(), get_number_app(st.app.as_str()),
                                result);

                            let send = SendMessage::new(key.clone());

                            let respo = send.sendNoTime(&value).await;

                            match respo {
                                Ok(e) => {}
                                Err(s) => {  }
                            }
                            Err(  "".to_string())
                        }
                        Err(e) => {


                            match bot::bot(&st, con, &self.map).await {
                                Ok(c) => {
                                    Ok(st.clone()) }
                                Err(e) => { Err(e) }
                            }


                        }
                    }






                } else {


                    let st = Status {
                        id: None,
                        st: "1".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone(),
                        name: Some( self.map.get("nodedouser").unwrap().clone() )
                    };

                    let insert = insert_status(&st, con.0).await;

                    match insert {
                        Ok(v) => {
                            if v == true {
                                match bot::bot(&st, con, &self.map).await {
                                    Ok(c) => { Ok(c) }
                                    Err(e) => { Err(e) }
                                }.expect("TODO: panic message");

                                Ok(st)
                            } else { Err(String::from("error ao inserir dados")) }
                        }
                        Err(e) => Err(e)
                    }
                }
            }
            Err(e) => { Err(String::from(e.kind.to_string())) }
        }
    }
    pub async fn run_list(&self, text: &String, con: &MongoDb<'_>) -> Result<Status, String> {
        let res = select_status(self.number.clone(), self.app.clone(), con.0).await;
        let key = std::env::var("KEY_API").unwrap();

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st: &Status = c.get(0).unwrap();

                            let new_status = Status {
                                id: st.id,
                                st: format!("{}-{}", st.st, text),
                                number: st.number.clone(),
                                app: st.app.clone(),
                                name: Some( self.map.get("nodedouser").unwrap().clone() )

                            };

                            match con.update_status(&new_status).await {
                                Ok(x) => { println!("atualizou o status") }
                                Err(e) => { println!("{:?}", e) }
                            };

                            let v = match bot::bot(&new_status, con, &self.map).await {
                                Ok(c) => { Ok(new_status) }
                                Err(e) => { Err(e) }
                            };

                            v






                } else {
                    let st = Status {
                        id: None,
                        st: "1".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone(),
                        name: Some( self.map.get("nodedouser").unwrap().clone() )

                    };

                    let insert = insert_status(&st, con.0).await;

                    match insert {
                        Ok(v) => {
                            if v == true {
                                match bot::bot(&st, con, &self.map).await {
                                    Ok(c) => { Ok(c) }
                                    Err(e) => { Err(e) }
                                }.expect("TODO: panic message");

                                Ok(st)
                            } else { Err(String::from("error ao inserir dados")) }
                        }
                        Err(e) => Err(e)
                    }
                }
            }
            Err(e) => { Err(String::from(e.kind.to_string())) }
        }
    }
    pub async fn run_button(&mut self, text: &String, con: &MongoDb<'_>) -> Result<Status, String> {
        let res = select_status(self.number.clone(), self.app.clone(), con.0).await;
        let key = std::env::var("KEY_API").unwrap();





        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st: &Status = c.get(0).unwrap();



                            let mut is_button_exit = false;


                            let newst = match text.as_str() {
                                "Voltar" => {
                                    let mut s = String::from(st.st.clone());
                                    let len = s.len();
                                    let (e, new_len) = s.split_at(len - 2);
                                    self.map.insert("voltar".to_string(), "true".to_string());
                                    format!("{}", e)
                                }

                                "Mais Informações" => {
                                    format!("{}-{}", st.st, 1)
                                }

                                "Menu principal" => {
                                    self.map.insert("voltar".to_string(), "true".to_string());
                                    "1".to_string()
                                }

                                "Encerrar conversa" => {
                                    is_button_exit = true;
                                    format!("{}-{}", st.st, 2)
                                }

                                "Reservar serviço" => {
                                    format!("{}-{}", st.st, 2)
                                }
                                _ => {
                                    "1".to_string()
                                }
                            };


                            let new_status = Status {
                                id: st.id,
                                st: newst,
                                number: st.number.clone(),
                                app: st.app.clone(),
                                name: Some(self.map.get("nodedouser").unwrap().clone())
                            };
                            match con.update_status(&new_status).await {
                                Ok(x) => { println!("atualizou o status") }
                                Err(e) => { println!("{:?}", e) }
                            };

                            match bot::bot(&new_status, con, &self.map).await {
                                Ok(c) => {
                                    if is_button_exit {
                                        con.delele_status(&new_status).await.unwrap();

                                        Ok(Status {
                                            id: new_status.id,
                                            st: "exit".to_string(),
                                            number: new_status.number,
                                            app: new_status.app,
                                            name: Some(self.map.get("nodedouser").unwrap().clone())
                                        })
                                    } else { Ok(new_status) }
                                }
                                Err(e) => { Err(e) }
                            }


                } else {
                    let st = Status {
                        id: None,
                        st: "1".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone(),
                        name: Some( self.map.get("nodedouser").unwrap().clone() )

                    };

                    let insert = insert_status(&st, con.0).await;

                    match insert {
                        Ok(v) => {
                            if v == true {
                                match bot::bot(&st, con, &self.map).await {
                                    Ok(c) => { Ok(c) }
                                    Err(e) => { Err(e) }
                                }.expect("TODO: panic message");

                                Ok(st)
                            } else { Err(String::from("error ao inserir dados")) }
                        }
                        Err(e) => Err(e)
                    }
                }
            }
            Err(e) => { Err(String::from(e.kind.to_string())) }
        }
    }
    pub async fn back(&mut self, con: &MongoDb<'_>) -> Result<bool, i32> {
        let res = select_status(self.number.clone(), self.app.clone(), con.0).await;

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st: &Status = c.get(0).unwrap();


                    let mut s = String::from(st.st.clone());
                    let len = s.len();
                    let (e, new_len) = s.split_at(len - 2);
                    self.map.insert("voltar".to_string(), "true".to_string());
                    let newst = format!("{}", e);

                    let new_status = Status {
                        id: st.id,
                        st: newst,
                        number: st.number.clone(),
                        app: st.app.clone(),
                        name: Some( self.map.get("nodedouser").unwrap().clone() )

                    };

                    match con.update_status(&new_status).await {
                        Ok(x) => {
                            match bot::bot(&new_status, con, &self.map).await {
                                Ok(c) => { Ok(true) }
                                Err(e) => { Err(1) }
                            }
                        }
                        Err(e) => { Err(0) }
                    }
                } else {
                    Err(1)
                }
            }
            Err(e) => { Err(2) }
        }
    }
}

pub fn status_not_key(status:String) -> Result<bool, &'static str> {

      match status.parse::<i32>() {
          Ok(x) => {
              if  x >= 1 {
                  Ok(true)
              }else {
                  Err("maior que um")
              }
          }
          Err(e) => {

                  Err("maior que um")


          }
      }

}