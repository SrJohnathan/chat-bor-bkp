use std::collections::HashMap;
use std::fmt::format;
use mongodb::error::Error;
use crate::chat::db_mongo::MongoDb;
use crate::chat::structs::status::Status;
use crate::model::mongo::{insert_status, select_status, update_status};


pub mod bot;
pub mod send_list_wp;
pub mod structs;
pub mod db_mongo;
pub mod factory_msg_send_text;


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

    pub async fn run(&self, con: &MongoDb<'_>) -> Result<Status, String> {
        let res = select_status(self.number.clone(), self.app.clone(), con.0).await;

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st: &Status = c.get(0).unwrap();


                    if st.st.len() >= 5 {
                        let new_status = Status {
                            id: st.id,
                            st: "1".to_string(),
                            number: st.number.clone(),
                            app: st.app.clone(),
                        };

                        match con.delele_status(&new_status).await {
                            Ok(x) => { println!("atualizou o status") }
                            Err(e) => { println!("{:?}", e) }
                        };
                        match bot::bot(&new_status, con, &self.map).await {
                            Ok(c) => { Ok(new_status) }
                            Err(e) => { Err(e) }
                        }
                    } else {
                        match bot::bot(&st, con, &self.map).await {
                            Ok(c) => { Ok(st.clone()) }
                            Err(e) => { Err(e) }
                        }
                    }
                } else {
                    let st = Status {
                        id: None,
                        st: "1".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone(),
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

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st: &Status = c.get(0).unwrap();


                    let new_status = Status {
                        id: st.id,
                        st: format!("{}-{}", st.st, text),
                        number: st.number.clone(),
                        app: st.app.clone(),
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
    pub async fn run_button(&mut self, text: &String, con: &MongoDb<'_>) -> Result<String, String> {
        let res = select_status(self.number.clone(), self.app.clone(), con.0).await;

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st: &Status = c.get(0).unwrap();


                    let newst = match text.as_str() {
                        "Voltar" => {
                            let mut s = String::from(st.st.clone());
                            let len = s.len();
                            let (e, new_len) = s.split_at(len - 2);
                            self.map.insert("voltar".to_string(), "true".to_string());
                            format!("{}", e)
                        }

                        "+ Informação" => {
                            format!("{}-{}", st.st, 1)
                        }

                        "Contratar" => {
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
                    };

                    match con.update_status(&new_status).await {
                        Ok(x) => { println!("atualizou o status") }
                        Err(e) => { println!("{:?}", e) }
                    };
                    match bot::bot(&new_status, con, &self.map).await {
                        Ok(c) => { Ok(c) }
                        Err(e) => { Err(e) }
                    }
                } else {
                    let st = Status {
                        id: None,
                        st: "1".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone(),
                    };

                    let insert = insert_status(&st, con.0).await;

                    match insert {
                        Ok(v) => {
                            if v == true {
                                match bot::bot(&st, con, &self.map).await {
                                    Ok(c) => { Ok(c) }
                                    Err(e) => { Err(e) }
                                }.expect("TODO: panic message");

                                Ok("ok".to_string())
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
                }else {
                    Err(1)
                }
            }
            Err(e) => { Err(2) }
        }
    }
}