use std::fmt::format;
use mongodb::error::Error;
use crate::chat::db_mongo::MongoDb;
use crate::chat::structs::status::Status;

use crate::model::mongo::{insert_status, select_status, update_status};


pub mod bot;
pub mod send_list_wp;
pub mod structs;
pub mod db_mongo;


#[derive(Clone)]
pub struct ChatWP {
    number: String,
    app:String
}


impl ChatWP {
    pub fn new(number: &str,app:&str) -> Self {
        ChatWP { number: number.to_string() ,app:app.to_string()}
    }

    pub async fn run(&self, con: &MongoDb<'_>) -> Result<String, String> {
        let res = select_status(self.number.clone(),self.app.clone(), con.0).await;

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st:&Status = c.get(0).unwrap();

                    println!("  fgdjhgdbgbfdhgjbfldhgb hjfbdglibfdhgb gdfilgnbdf  {}",st.st.len()  );

                    if st.st.len() >= 5 {

                        let new_status = Status{
                            id: st.id,
                            st: "1".to_string(),
                            number: st.number.clone(),
                            app: st.app.clone()
                        };

                      match con.update_status(&new_status).await {
                          Ok(x) => {println!("atualizou o status")}
                          Err(e) => { println!("{:?}",e) }
                      };

                    }

                   match  bot::bot(&st,con).await {
                       Ok(c) => {Ok(c)}
                       Err(e) => {Err(e)}
                   }
                } else {
                    let st =  Status{
                        id: None,
                        st: "1".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone()
                    };

                    let insert =   insert_status(&st,con.0).await;

                    match insert {
                        Ok(v) => {  if  v == true {


                            match bot::bot(&st,con).await {
                                Ok(c) => { Ok(c) }
                                Err(e) => { Err(e) }
                            }.expect("TODO: panic message");

                           Ok("ok".to_string())



                        } else {  Err(String::from("error ao inserir dados"))  }  }
                        Err(e) => Err(e)
                    }
                }
            }
            Err(e) => { Err(String::from(e.kind.to_string())) }
        }
    }
    pub async fn run_list(&self, text:&String,con: &MongoDb<'_>) -> Result<String, String> {
        let res = select_status(self.number.clone(),self.app.clone(), con.0).await;

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st:&Status = c.get(0).unwrap();


                   let   newStatus  = Status{
                       id: st.id,
                       st: format!("{}-{}",st.st,text),
                       number: st.number.clone(),
                       app: st.app.clone()
                   };

                    update_status(&newStatus,con.0).await.unwrap();

                    match  bot::bot(&newStatus,con).await {
                        Ok(c) => {Ok(c)}
                        Err(e) => {Err(e)}
                    }
                } else {
                    let st =  Status{
                        id: None,
                        st: "1".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone()
                    };

                    let insert =   insert_status(&st,con.0).await;

                    match insert {
                        Ok(v) => {  if  v == true {


                            match bot::bot(&st,con).await {
                                Ok(c) => { Ok(c) }
                                Err(e) => { Err(e) }
                            }.expect("TODO: panic message");

                            Ok("ok".to_string())



                        } else {  Err(String::from("error ao inserir dados"))  }  }
                        Err(e) => Err(e)
                    }
                }
            }
            Err(e) => { Err(String::from(e.kind.to_string())) }
        }
    }
    pub async fn run_button(&self,text:&String ,con: &MongoDb<'_>) -> Result<String, String> {
        let res = select_status(self.number.clone(),self.app.clone(), con.0).await;

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st:&Status = c.get(0).unwrap();


                    let  newst =  match text.as_str()  {
                        "Voltar" =>  {

                            let mut s = String::from(st.st.clone());
                            let len = s.len();
                            let (e, new_len) = s.split_at(len - 2);
                            format!("{}",e)
                        }

                        "+ Informação" =>  {
                            format!("{}-{}",st.st,1)
                        }

                        "Contratar" =>  {
                            format!("{}-{}",st.st,2)
                        }
                        _ => {
                            "1".to_string()
                        }
                    };
                    let new_status = Status{
                        id: st.id,
                        st: newst,
                        number: st.number.clone(),
                        app: st.app.clone()
                    };

                    update_status(&new_status, con.0).await.unwrap();

                    match  bot::bot(&new_status, con).await {
                        Ok(c) => { Ok(c) }
                        Err(e) => { Err(e) }
                    }

                } else {
                    let st =  Status{
                        id: None,
                        st: "1".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone()
                    };

                    let insert =   insert_status(&st,con.0).await;

                    match insert {
                        Ok(v) => {  if  v == true {


                            match bot::bot(&st,con).await {
                                Ok(c) => { Ok(c) }
                                Err(e) => { Err(e) }
                            }.expect("TODO: panic message");

                            Ok("ok".to_string())



                        } else {  Err(String::from("error ao inserir dados"))  }  }
                        Err(e) => Err(e)
                    }
                }
            }
            Err(e) => { Err(String::from(e.kind.to_string())) }
        }
    }
}