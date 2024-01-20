


pub mod list_mongo;
pub mod text_mongo;
pub mod text_buttons;
pub mod status;

use mongodb::bson::oid::ObjectId;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use crate::chat::factory_msg_send_text::TypeMidia;
use crate::chat::structs::list_mongo::ListMongo;
use crate::chat::structs::text_buttons::{ContentMedia, ContentText, TextButtons};
use crate::chat::structs::text_mongo::TextMongo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat<T> {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub index: String,
    pub app:String,
    pub data: T,
    #[serde(rename = "type")]
    pub type_field: String,
    pub midia:bool,
    pub type_midia:TypeMidia
}

pub enum ChatDataType {
    Null ,
    Text(Vec<Chat<TextMongo>>),
    List(Vec<Chat<ListMongo>>),
    ButtonMidia(Vec<Chat<TextButtons<ContentMedia>>>),
    ButtonText(Vec<Chat<TextButtons<ContentText>>>)


}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClientKeyBot {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: std::option::Option<ObjectId>,
    pub number: String,
    pub app:String,
    pub data: String,
    pub keys: Vec<String>,
    pub name: Option<String>,
    pub show: bool

}