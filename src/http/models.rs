use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use reqwest::{Client, Response, Error, StatusCode};
use crate::chat::send_list_wp::SendWP;
use crate::cofg::{HOST_API_GUPSHUP, MESSAGE_PATH_GUPSHUP};


pub enum Channels {
    whatsapp,
    sms,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct MessageGupshup {
    pub r#type: String,
    pub text: Option<String>,
    pub originalUrl: Option<String>,
    pub previewUrl: Option<String>,
    pub url: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct SendMessage {
    pub api_key: String,


}


impl SendMessage {
    pub fn  new( api_key: String) -> Self {
        SendMessage{api_key}
    }
    pub async fn send<T:serde::Serialize>(&self,body: SendWP<T>) -> Result<(StatusCode, String), Error> {


       let message =  body.to_json().await;


        let params =
            [("channel", "whatsapp"),
                ("source", body.source.as_str()),
                ("destination", body.destination.as_str()),
                ("message", message.as_str() ),
                ("disablePreview", "false"),
                ("src.name", body.src_name.as_str() )];

        let req: Client = Client::new();
        let response = req.post(format!("{}{}", HOST_API_GUPSHUP, MESSAGE_PATH_GUPSHUP))
            .header("apikey", self.api_key.as_str())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send().await;

        match response {
            Ok(c) => Ok((c.status(), c.text().await.unwrap())),
            Err(e) => Err(e)
        }
    }
}


#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct ParentMessage<T> {
    pub app: String,
    pub timestamp: isize,
    pub version: i32,
    pub r#type: String,
    pub payload: T,
}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Read {
    pub ts: i32,
}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Sent {
    pub ts: i32,
}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Delivered {
    pub ts: i32,
}
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Enqueued {
    pub whatsappMessageId: String,
    pub r#type: String,

}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Failed {
    pub code: String,
    pub reason: String,

}
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct MessageEvent<T> {
    pub id: String,
    pub gsId: Option<String>,
    pub r#type: String,
    pub destination: String,
    pub payload: T,
}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct MessageGP<T> {
    pub id: String,
    pub r#type: String,  // "text"|"image"|"file"|"audio"|"video"|"contact"|"location"|"button_reply"|"list_reply",
    pub source: String,
    pub payload: T,
    pub sender: Sender,
    pub context: Option<Context>,
}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Sender {
    pub phone: String,
    pub name: String,
    pub country_code: String,
    pub dial_code: String,

}
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Context {
    pub id: String,
    pub gsId: String,

}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Text {
    pub text: String,
    pub r#type:Option<String>

}
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Image {
   pub caption: String,
   pub url: String,
   pub contentType: String,
   pub urlExpiry: isize
}
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Video {
   pub caption: String,
   pub url: String,
   pub contentType: String,
   pub urlExpiry: isize
}
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct File {
   pub caption: String,
   pub name:String,
   pub url: String,
   pub contentType: String,
   pub urlExpiry: isize
}
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Audio {
   pub url: String,
   pub contentType: String,
   pub urlExpiry: isize
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct ListReply {
    pub title:String,
    pub id: String,
    pub reply:String,
    pub postbackText:String,
    pub description: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct ButtonReply {
   pub title: String,
   pub id: String,
   pub reply:String
}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Location {
    pub longitude: String,
    pub latitude: String
}




#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct SidToken {
    pub sid: String,
    pub token: String,
}

#[derive(Deserialize, Debug, Serialize, Clone, JsonSchema)]
pub struct Cliente {
    pub idcliente: i32,
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub endereco: Option<String>,
    pub cpfcnpj: Option<String>,
    pub sid: Option<String>,
    pub token: Option<String>,

}

#[derive(Deserialize, Debug, Serialize, Clone, JsonSchema)]
pub struct Whatsapp {
    pub idp: i32,
    pub numero: Option<String>,
    pub idcliente: i32,
    pub config: i32,
    pub site: Option<String>,
    pub nome_empresa: Option<String>,
    pub id_empresa_face: Option<String>,
    pub type_empresa: Option<String>,
    pub n_maximo: Option<String>,
    pub verify: i32,
    pub type_c: i32,
    pub token: Option<String>,
    pub hook: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone, JsonSchema)]
pub struct Value {
    pub to: String,
    pub path: Option<String>,
    pub text: Option<String>,
    pub username: Option<String>,

}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize, Clone, JsonSchema)]
pub struct SendTwilio {
    pub From: String,
    pub Body: String,
    pub To: String,
    pub MediaUrl: Option<String>,
}