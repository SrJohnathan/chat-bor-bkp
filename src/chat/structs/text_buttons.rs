
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextButtons<T> {
    #[serde(rename = "type")]
    pub type_field: String, // "text" | "image" | "video" | "document"
    pub msgid: String,
    pub content: T,
    pub options: Vec<OptionB>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMedia {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentText {
    #[serde(rename = "type")]
    pub type_field: String,
    pub header: String,
    pub text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionB {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
}