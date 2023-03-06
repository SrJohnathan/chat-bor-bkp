use std::ops::Deref;
use std::sync::mpsc::{channel, SendError};

use std::time::Duration;
use serde_derive::{Deserialize, Serialize};
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use tokio::sync::{mpsc, oneshot::{Receiver, Sender}, oneshot};
use tokio::task::JoinHandle;
use tokio::time;

pub const HOST_API_GUPSHUP:&str = "https://api.gupshup.io";
pub const MESSAGE_PATH_GUPSHUP:&str ="/sm/api/v1/msg";

pub const API_DEV:&str ="1wnuo9xzw0xlnavgtd0zjkqlwv7yci9x";
pub const API_PRODU:&str = "ku8gzeihiztucp71pog5xoipestl5abp";

pub fn get_number_app(app:&str) -> &str {

     match app {
         "WhatsAppSTWpt" => "351253930233",
         _ => "917834811114",
    }

}

pub fn get_app_app(app:&str) -> &str {

    match app {
        "WhatsAppSTWpt" => API_PRODU,
        _ => API_DEV,
    }

}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct NewJob{
   pub number:String,
   pub etapa:String,
   pub time: i32
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




