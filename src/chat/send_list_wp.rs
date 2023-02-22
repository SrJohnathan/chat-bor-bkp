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
    pub async fn to_json(&self) -> String {
        serde_json::to_string(&self.message).unwrap()
    }
    pub fn urlencoded(&self) -> String {
        serde_urlencoded::to_string(self).unwrap()
    }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub body: String,
    pub msgid: String,
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
