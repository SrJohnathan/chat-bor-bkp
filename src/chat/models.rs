use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendListWP {
    pub channel: String,
    pub source: String,
    pub destination: String,
    #[serde(rename = "src.name")]
    pub src_name: String,
    pub message: Message,
}


impl SendListWP {
    pub fn new (app:&str,number:&str,myphone:&str,msg:Message) ->Self {
        Self {
            channel: "whatsapp".to_string(),
            source: myphone.to_string(),
            destination: number.to_string(),
            src_name: app.to_string(),
            message: msg,
        }
    }
    pub fn toJson(&self) -> String {
        serde_json::to_string(self).unwrap()
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
    pub options: Vec<Option>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub description: String,
    pub postback_text: String,
}
