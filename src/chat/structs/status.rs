use mongodb::bson::oid::ObjectId;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Status {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub st:String,
    pub number:String,
    pub app:String
}