use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiverInstagram {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub time: i64,
    pub id: String,
    pub messaging: Vec<Messaging>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Messaging {
    pub sender: Sender,
    pub recipient: Recipient,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<Read>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipient {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub mid: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Read {
    pub mid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendFBIG<T> {
    pub channel: String,  // page  || instagram
    pub recipient: String,
    pub access_token:String,
    pub message: T,
    pub page_id:String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FBIG<T> {
    pub recipient: Recipient,
    pub message: T,

}

impl <T:Sized + serde::Serialize>SendFBIG<T> {
    pub fn new (page_id:String,number:String,msg:T,channel:String,access_token:String) ->Self {
        Self {
            channel,
            recipient: number,
            access_token,
            page_id,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FBIGText {
    pub recipient: IdFbIg,
    pub message: Text,
    pub access_token:String,

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdFbIg {
    pub id: String,

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub text: String,

}

