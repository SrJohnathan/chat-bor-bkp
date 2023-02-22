use std::future::Future;
use reqwest::{Error, StatusCode};
use tokio::fs;
use crate::model::models::Status;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::chat::send_list_wp;
use crate::chat::send_list_wp::{Message, SendWP};
use crate::cofg::API_DEV;
use crate::http::models::SendMessage;

pub async fn bot(st: &Status) {
    let tmp: Vec<&str> = st.st.split("-").collect();
    let ar: Vec<String> = tmp.iter().map(|c| c.replace("-", "")).filter(|c| c.as_str() != "").collect();

    println!("{:?}", ar);

    let mut body = String::new();
    let data = json_to_data().await.unwrap();
    for i in ar {
        match i.parse().unwrap() {
            0 => {
                let bot = data.data.get(0).unwrap();

                let message = send_list_wp::Message {
                    type_field: bot.type_field.to_string(),
                    title: "Serviços".to_string(),
                    body: bot.body.to_string(),
                    msgid: "list1".to_string(),
                    global_buttons: vec![
                        send_list_wp::GlobalButton {
                            type_field: "text".to_string(),
                            title: bot.button_menu.title.to_string(),
                        }
                    ],
                    items: vec![send_list_wp::Item {
                        title: bot.payload.title.to_string(),
                        subtitle: bot.payload.title.to_string(),
                        options: bot.payload.itens.iter().map(|c| send_list_wp::Optio {
                            type_field: c.type_field.to_string(),
                            title: c.title.to_string(),
                            description: Default::default(),
                            postback_text: Default::default(),
                        }).collect(),
                    }],
                };

                let g:SendWP<Message> = SendWP::new(
                    st.app.as_str(),
                    st.number.as_str(), "917384811114", message);

                println!("{:?}", g);

                let send = SendMessage::new( API_DEV.to_string());
                match send.send(g).await {
                    Ok(c) => { println!("{:?}", c) }
                    Err(e) => { println!("{:?}", e.to_string()) }
                }
            }
            _ => {}
        }
    }
}


async fn json_to_data() -> Result<DataJson, String> {
    let data = {
        let d = fs::read_to_string(r"./data.json").await;

        match d {
            Ok(v) => {
                let dat: DataJson = serde_json::from_str(&v).unwrap();
                Ok(dat)
            }
            Err(e) => Err(e.kind().to_string())
        }
    };

    data
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataJson {
    pub bot: Bot,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    #[serde(rename = "0")]
    pub n0: i64,
    #[serde(rename = "0-1")]
    pub n0_1: i64,
}

impl Bot {
    pub fn get(&self, n: i32) -> i64 {
        match n {
            0 => self.n0,
            1 => self.n0_1,
            _ => 0
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    #[serde(rename = "type")]
    pub type_field: String,
    pub body: String,
    pub payload: Payload,
    pub button_menu: ButtonMenu,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub title: String,
    pub itens: Vec<Iten>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iten {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonMenu {
    pub title: String,
}
