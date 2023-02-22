use mongodb::error::Error;
use crate::model::models::Status;
use crate::model::mongo::{insert_status, MongoDb, select_status};


pub mod bot;
pub mod send_list_wp;



#[derive(Clone)]
pub struct Chat {
    number: String,
    app:String
}


impl Chat {
    pub fn new(number: &str,app:&str) -> Self {
        Chat { number: number.to_string() ,app:app.to_string()}
    }

    pub async fn run(&self, con: &MongoDb<'_>) -> Result<String, String> {
        let res = select_status(self.number.clone(),self.app.clone(), con.0).await;

        match res {
            Ok(c) => {
                if c.len() > 0 {
                    let st = c.get(0).unwrap();


                    bot::bot(&st).await;

                    Ok(String::from("Error em carregar dados de Status"))
                } else {
                    let st =  Status{
                        id: None,
                        st: "0-".to_string(),
                        number: self.number.clone(),
                        app: self.app.clone()
                    };

                    let insert =   insert_status(&st,con.0).await;

                    match insert {
                        Ok(v) => {  if  v == true {


                           Ok("OK".to_string())



                        } else {  Err(String::from("error ao inserir dados"))  }  }
                        Err(e) => Err(e)
                    }
                }
            }
            Err(e) => { Err(String::from(e.kind.to_string())) }
        }
    }
}