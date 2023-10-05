use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use reqwest::{Client, Response, Error, StatusCode};
use crate::chat::send_list_wp::SendWP;
use crate::cofg::{get_app_app, HOST_API_GUPSHUP, MESSAGE_PATH_GUPSHUP};
use mongodb::bson::oid::ObjectId;
use crate::chat::models_instagram::{FBIG, Recipient, SendFBIG};

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
    pub fn new(api_key: String) -> Self {
        SendMessage { api_key }
    }


    pub async fn sendTemplate<T: serde::Serialize + Send + 'static>(&self, vec: &SendWP<T>) -> Result<String, String> {
        let req: Client = Client::new();

        let message = vec.to_json();

        let params =
            [("channel", "whatsapp"),
                ("source", vec.source.as_str()),
                ("destination", vec.destination.as_str()),
                ("template", &message),
                ("disablePreview", "false"),
                ("src.name", vec.src_name.as_str())];




        let response = req.post(format!("{}{}", HOST_API_GUPSHUP, MESSAGE_PATH_GUPSHUP))
            .header("apikey", get_app_app(vec.src_name.as_str()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            // .header("Content-Length", content_length.to_string())
            .form(&params)
            .send().await;





        match response {
            Ok(x) => {
                Ok(x.text().await.unwrap())
            }
            Err(e) => { Err(e.to_string()) }
        }
    }


    pub async fn sendNoTime<T: serde::Serialize + Send + 'static>(&self, vec: &SendWP<T>) -> Result<String, String> {
        let req: Client = Client::new();

        let message = vec.to_json();

        let params =
            [("channel", "whatsapp"),
                ("source", vec.source.as_str()),
                ("destination", vec.destination.as_str()),
                ("message", &message),
                ("disablePreview", "false"),
                ("src.name", vec.src_name.as_str())];




        let response = req.post(format!("{}{}", HOST_API_GUPSHUP, MESSAGE_PATH_GUPSHUP))
            .header("apikey", get_app_app(vec.src_name.as_str()))
            .header("Content-Type", "application/x-www-form-urlencoded")
           // .header("Content-Length", content_length.to_string())
            .form(&params)
            .send().await;





        match response {
            Ok(x) => {
                Ok(x.text().await.unwrap())
            }
            Err(e) => { Err(e.to_string()) }
        }
    }

    pub async fn send<T: serde::Serialize + Send + 'static + std::marker::Sync>(&self, vec: Vec<SendWP<T>>) {
        let req: Client = Client::new();


        tokio::spawn(async move {
            for body in vec {
                let message = body.to_json();


                let params =
                    [("channel", "whatsapp"),
                        ("source", body.source.as_str()),
                        ("destination", body.destination.as_str()),
                        ("message", &message),
                        ("disablePreview", "false"),
                        ("src.name", body.src_name.as_str())];

                tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;

                if is_bot(&req, body.src_name.clone(), body.destination.clone()).await {

                    let response = req.post(format!("{}{}", HOST_API_GUPSHUP, MESSAGE_PATH_GUPSHUP))
                        .header("apikey", get_app_app(body.src_name.as_str()))
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .form(&params)
                        .send().await;

                    match response {
                        Ok(x) => {
                            let response = req.post("https://siga-telecom.herokuapp.com/api/v1/whatsapp/webHookSocketAlt")
                                // .header("Content-Type", "application/json")
                                .json(&body)
                                .send().await;
                            match response {
                                Ok(e) => {}
                                Err(s) => {}
                            }


                            println!("{:?}", x.text().await.unwrap())
                        }
                        Err(e) => { println!("{:?}", e.to_string()) }
                    }
                }


            }
        });



    }
    pub async fn send_instagram<T: serde::Serialize + Send + 'static + std::marker::Sync + std::fmt::Debug>(&self, vec: Vec<SendFBIG<T>>) {
        let req: Client = Client::new();


        tokio::spawn(async move {
            for body in vec {
                let msg = FBIG { recipient: Recipient { id: body.recipient }, message: body.message };

                let response = req.post(format!("{}{}/messages?access_token={}", "https://graph.facebook.com/v16.0/", body.page_id.as_str(), body.access_token.as_str()))
                    .json(&msg)
                    .send().await;

                match response {
                    Ok(x) => {
                        println!("{:?}", x.text().await.unwrap())
                    }
                    Err(e) => { println!("{:?}", e.to_string()) }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
            }
        });
    }
    pub async fn send_facebook<T: serde::Serialize + Send + 'static>(&self, vec: Vec<SendWP<T>>) {}
}


async fn is_bot(req: &reqwest::Client, app: String, number: String) -> bool {
    let tmp = format!("https://siga-telecom.herokuapp.com/api/v1/whatsapp/isbot/{}/{}", app, number);

    // Realiza a solicitação HTTP e aguarda a resposta
    let response = req.get(&tmp).send().await;

    // Verifica se houve algum erro na solicitação HTTP
    match response {
        Ok(resp) => {
            // Verifica se o status da resposta é bem-sucedido (código 2xx)
            if resp.status().is_success() {
                // Lê o conteúdo da resposta como texto
                let body = resp.text().await;

                // Verifica se houve algum erro ao obter o conteúdo da resposta
                match body {
                    Ok(text) => {
                        // Converte o texto em um bool
                        if text.trim() == "true" {
                            true
                        } else if text.trim() == "false" {
                            false
                        } else {
                            // Resposta inválida da API, trata o erro como preferir
                            eprintln!("Resposta inválida da API: {}", text);
                            false
                        }
                    }
                    Err(err) => {
                        // Tratamento de erro ao obter o corpo da resposta
                        eprintln!("Erro ao obter o corpo da resposta: {}", err);
                        false // ou tratar o erro de outra forma, dependendo do caso
                    }
                }
            } else {
                // Tratamento de erro para resposta com status diferente de 2xx (erro HTTP)
                eprintln!("Erro na resposta HTTP: {:?}", resp.status());
                false // ou tratar o erro de outra forma, dependendo do caso
            }
        }
        Err(err) => {
            // Tratamento de erro na solicitação HTTP
            eprintln!("Erro na solicitação HTTP: {}", err);
            false // ou tratar o erro de outra forma, dependendo do caso
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
    pub code: isize,
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
    pub r#type: String,
    // "text"|"image"|"file"|"audio"|"video"|"contact"|"location"|"button_reply"|"list_reply",
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
    pub r#type: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Image {
    pub caption: Option<String>,
    pub url: String,
    pub contentType: String,
    pub urlExpiry: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Video {
    pub caption: String,
    pub url: String,
    pub contentType: String,
    pub urlExpiry: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct File {
    pub caption: Option<String>,
    pub name: String,
    pub url: String,
    pub contentType: String,
    pub urlExpiry: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Audio {
    pub url: String,
    pub contentType: String,
    pub urlExpiry: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct ListReply {
    pub title: String,
    pub id: String,
    pub reply: String,
    pub postbackText: String,
    pub description: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct ButtonReply {
    pub title: String,
    pub id: String,
    pub reply: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, JsonSchema)]
pub struct Location {
    pub longitude: String,
    pub latitude: String,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacebookToken {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "data_access_expiration_time")]
    pub data_access_expiration_time: String,
    #[serde(rename = "expires_in")]
    pub expires_in: String,
    #[serde(rename = "long_lived_token")]
    pub long_lived_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}
/*
curl -i -X GET \
"https://graph.facebook.com/v16.0/me/accounts?access_token=EAAHd7pJH78sBAG8oA3ZAkge3xh280oYwVUZCVwEQ18eP323djNMbvHHoecnuywLIF2lVpnN5HoUfHQr7Pxl3ZCXwe7xE6NbmFJhxYzPtLMpBR0ZCUJ7GACzN0rZA1pBK6YhZAQuz2uTzYqMVpTO9kk4AAriATS0AiLIwyd0sHrNzc4Sx3aRG8KmdCA1FLrvYYJy9zd41OnoK5FZB3ipHltDsSlPZBZApZBHs0ZD"
*/
