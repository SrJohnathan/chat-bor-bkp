use mongodb::error::Error;
use crate::chat::db_mongo::MongoDb;
use crate::chat::structs::status::Status;

use crate::model::mongo::{insert_status, select_status};


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
}