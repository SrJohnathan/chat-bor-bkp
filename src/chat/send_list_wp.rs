use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendWP<T> {
    pub channel: String,
    pub source: String,
    pub destination: String,
    #[serde(rename = "src.name")]
    pub src_name: String,
    pub message: T,
}


impl <T:Sized + serde::Serialize>SendWP<T> {
    pub fn new (app:&str,number:&str,myphone:&str,msg:T) ->Self {
        Self {
            channel: "whatsapp".to_string(),
            source: myphone.to_string(),
            destination: number.to_string(),
            src_name: app.to_string(),
            message: msg,
        }
    }
    pub  fn to_json(&self) -> String {
        serde_json::to_string(&self.message).unwrap()
    }
    pub fn urlencoded(&self) -> String {

        serde_urlencoded::to_string(self).unwrap()
    }
}



//TEMPLATE
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateText {
    //#[serde(rename = "type")]
    pub id: String,
    pub params: Vec<String>,

}



//TEXT

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageText {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,

}


// MIDIA IMAGE

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMidia {
    #[serde(rename = "type")]
    pub type_field: String,
    pub original_url: String,
    pub preview_url: String,
    pub caption: String,
}

//VIDEO - DOCUMENT - AUDIO

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MidiaType {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub filename: Option<String>,
}


// Button

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonWP<T> {
    #[serde(rename = "type")]
    pub type_field: String,
    pub msgid: String,
    pub content: T, // ContentBT  para botoẽs do tipo texto || ContentMD para midia
    pub options: Vec<OptionBT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentBT {
    #[serde(rename = "type")]
    pub type_field: String,
    pub header: String,
    pub text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMD {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionBT {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
}



//LIST

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub body: String,
    pub msgid: Option<String>,
    pub global_buttons: Vec<GlobalButton>,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalButton {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub title: String,
    pub subtitle: String,
    pub options: Vec<Optio>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optio {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub description: Option<String>,
    pub postback_text: Option<String>,
}
