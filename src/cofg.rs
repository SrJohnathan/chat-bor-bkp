use std::ops::Deref;
use std::sync::mpsc::{channel, SendError};

use std::time::Duration;
use chrono::{ NaiveDateTime};
use reqwest::{Error, Response};
use serde_derive::{Deserialize, Serialize};
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use serde_json::Value;
use tokio::sync::{mpsc, oneshot::{Receiver, Sender}, oneshot};

pub const HOST_API_GUPSHUP:&str = "https://api.gupshup.io";
pub const HOST_API_GUPSHUP_NO_HTTPS:&str = "http://api.gupshup.io";
pub const MESSAGE_PATH_GUPSHUP:&str ="/sm/api/v1/msg";

pub const API_DEV:&str ="1wnuo9xzw0xlnavgtd0zjkqlwv7yci9x";
pub const API_PRODU:&str = "ku8gzeihiztucp71pog5xoipestl5abp";

pub fn get_number_app(app:&str) -> &str {

     match app {
         "WhatsAppSTWpt" => "351253930233",
         "BusinessSTWpt" => "351926683992",
         _ => "917834811114",
    }

}

pub fn get_app_app(app:&str) -> &str {

    match app {
        "WhatsAppSTWpt" => API_PRODU,
        "BusinessSTWpt" => API_PRODU,
        _ => API_DEV,
    }

}


pub fn get_app_id(app:&str) -> &str {

    match app {
        "WhatsAppSTWpt" => "2ba40208-eeeb-453f-9a76-66510031344f",
        "BusinessSTWpt" => "f60e78d0-c1a9-4d9d-b2ec-87a46928cc9a",
        _ => "",
    }

}


#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct NewJob{
   pub number:String,
   pub etapa:String,
   pub time: i32,
   pub  app:String
}


pub struct JobWP<'r>(pub(crate) & 'r Sender<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JobWP<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let res = request.guard::<&State<Sender<String>>>().await;
        res.map(|c|  JobWP::new(c) )
    }
}



impl <'r>JobWP<'r> {

    pub fn new (sender: &  'r Sender<String>) -> Self {
        JobWP(sender)
    }

}



pub async  fn api_leads(string: &String) -> Result<Leads, String> {
    let req = reqwest::Client::new();

 let res =    req.get( format!("https://api-sigaa.herokuapp.com/api/v1/leads/number/wp/{}",string))
        .send().await;

    match res  {
        Ok(x) => {

             if let Ok(f) =  x.json::<Option<Leads>>().await {


                 match  f {
                     None =>  Err("nullo".to_string()),
                     Some(array) =>  Ok(array)
                 }

           } else {

                 Err("nullo".to_string())
             }

        }
        Err(e) => Err(e.to_string())
    }



}

#[derive(Debug, Serialize, Deserialize)]
 pub struct Leads {
    id: String,
    name: String,
    email: String,
    phone: String,
    details: Option<Value>,
    service_id: String,
    status: String,
    status_type: String,
}

