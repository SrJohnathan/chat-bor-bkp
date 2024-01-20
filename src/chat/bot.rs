use std::collections::HashMap;

use serde_json::Value;
use crate::chat::db_mongo::MongoDb;
use crate::chat::factory_msg_send_text::TypeMidia;
use crate::chat::models_instagram::{Attachment, ListButtonIG, Midia, Payload, QuickReply, SendFBIG, Text};
use crate::chat::send_list_wp;
use crate::chat::send_list_wp::{ButtonWP, ContentBT, ContentMD, GlobalButton, ImageMidia, Item, Message, MessageText, MidiaType, OptionBT, SendWP};
use crate::chat::structs::{Chat, ChatDataType, ClientKeyBot};
use crate::chat::structs::list_mongo::{ButtonMenu, Iten, ListMongo};
use crate::chat::structs::status::Status;
use crate::chat::structs::text_buttons::{ContentText, OptionB, TextButtons};
use crate::chat::structs::text_mongo::{Body, TextMongo};
use crate::cofg::{API_DEV, API_PRODU, get_number_app};
use crate::http::models::SendMessage;

fn description_list_1(i: i32, st: &str) -> Option<String> {
    let e = match st {
        "1" => {
            match i {
                0 => "Matrículas em escolas ou universidades no estrangeiro",
                1 => "Auxiliar na solicitação de qualquer tipo de visto",
                2 => "Pesquisa de alojamento, inclusive antes de chegar ao seu destino",
                3 => "Até 10% mais baratos do que o preço normal de mercado",
                4 => "Documentos necessários para residir legalmente no estrangeiro",
                5 => "Tudo financiado para estudar no estrangeiro",
                _ => Default::default(),
            }
        }

        _ => {
            match i {
                0 => "Selecione e receba o link para mais informações.",
                1 => "Selecione e receba o link para contratar este serviço.",
                2 => "Selecione para voltar ao menu inicial",
                _ => Default::default(),
            }
        }
    };


    Some(e.to_string())
}


pub async fn bot(st: &Status, db: &MongoDb<'_>, map: &HashMap<String, String>) -> Result<String, String> {
    let tmp: Vec<&str> = st.st.split("-").collect();
    let ar: Vec<String> = tmp.iter().map(|c| c.replace("-", "")).filter(|c| c.as_str() != "").collect();
    let key = std::env::var("KEY_API").unwrap();


    match st.app.as_str() {
        "instagram" => {
            let g = match db.get_chat(&st.st, &st.app).await {
                Ok(c) => {
                    let token = db.get_token_facebook().await.unwrap();

                    let i = match c {
                        ChatDataType::Null => {
                            Err("null".to_string())
                        }
                        ChatDataType::Text(text) => {
                            let mut vec = Vec::new();


                            for button in text {
                                let result = if button.midia {
                                    match button.type_midia {
                                        TypeMidia::AUDIO => {
                                            serde_json::to_value(
                                                Midia {
                                                    attachment: Attachment {
                                                        type_field: "audio".to_string(),
                                                        payload: Payload { url: button.type_field },
                                                        // is_reusable: true
                                                    }
                                                }
                                            ).unwrap()
                                        }

                                        TypeMidia::NULL => { todo!() }
                                        TypeMidia::IMAGE => {
                                            serde_json::to_value(
                                                Midia {
                                                    attachment: Attachment {
                                                        type_field: "image".to_string(),
                                                        payload: Payload { url: button.type_field },
                                                        // is_reusable: true
                                                    }
                                                }
                                            ).unwrap()
                                        }
                                        TypeMidia::DOCUMENT => { todo!() }
                                        TypeMidia::VIDEO => {
                                            serde_json::to_value(
                                                Midia {
                                                    attachment: Attachment {
                                                        type_field: "video".to_string(),
                                                        payload: Payload { url: button.type_field },
                                                        //  is_reusable: true
                                                    }
                                                }
                                            ).unwrap()
                                        }
                                    }
                                } else {
                                    serde_json::to_value(
                                        Text { text: button.data.body.text }
                                    ).unwrap()
                                };

                                let page = token.page.clone().unwrap();
                                let value: SendFBIG<Value> = SendFBIG::new(
                                    page,
                                    st.number.clone(),
                                    result,
                                    st.app.clone(),
                                    token.page_token.clone().unwrap());

                                vec.push(value);
                            }


                            Ok(vec)
                        }
                        ChatDataType::List(list) => {
                            let mut vec = Vec::new();

                            for bo in list {
                                let bot = bo.data;




                                let mut it:Vec<Vec<QuickReply>> = bot.payload.iter().enumerate().map(|(i, v)| {
                                    v.itens.iter().enumerate().map(|(e, c)|
                                        QuickReply {
                                            content_type: "text".to_string(),
                                            title: c.title.to_string(),
                                            payload: format!("n{}", e as i32),
                                        }
                                    ).collect()

                                    /*  send_list_wp::Item {
                                          title: v.title.to_string(),
                                          subtitle: v.title.to_string(),
                                          options: v.itens.iter().enumerate().map(|(e, c)| send_list_wp::Optio {
                                              type_field: c.type_field.to_string(),
                                              title: c.title.to_string(),
                                              description: description_list_1(e as i32,st.st.as_str()),
                                              postback_text: Some(  format!("n{}",  e as i32)),
                                          }).collect(),
                                      } */
                                }).collect();

                                /*   for i in 0..bot.button_menu.len() {
                                       let item: &Item = it.get(i).expect("");
                                       let btn: &GlobalButton = gb.get(i).expect("");
                                   } */

                                let mut text_final = if map.contains_key("voltar") {
                                    "Em que posso lhe ajudar?".to_string()
                                } else {
                                    match map.get("nodedouser") {
                                        None => { bot.body }
                                        Some(x) => { bot.body.replace("nodedouser", x.as_str()) }
                                    }
                                };

                                let dat = {
                                    if bot.show.unwrap() {
                                        serde_json::to_value(
                                            ListButtonIG{ text: text_final, quick_replies :it[0].clone() }
                                        ).unwrap()
                                    } else {
                                        if bo.midia {
                                            match bo.type_midia {
                                                TypeMidia::AUDIO => {
                                                    serde_json::to_value(
                                                        Midia {
                                                            attachment: Attachment {
                                                                type_field: "audio".to_string(),
                                                                payload: Payload { url: bo.type_field },
                                                                // is_reusable: true
                                                            }
                                                        }
                                                    ).unwrap()
                                                }

                                                TypeMidia::NULL => { todo!() }
                                                TypeMidia::IMAGE => {
                                                    serde_json::to_value(
                                                        Midia {
                                                            attachment: Attachment {
                                                                type_field: "image".to_string(),
                                                                payload: Payload { url: bo.type_field },
                                                                // is_reusable: true
                                                            }
                                                        }
                                                    ).unwrap()
                                                }
                                                TypeMidia::DOCUMENT => { todo!() }
                                                TypeMidia::VIDEO => {
                                                    serde_json::to_value(
                                                        Midia {
                                                            attachment: Attachment {
                                                                type_field: "video".to_string(),
                                                                payload: Payload { url: bo.type_field },
                                                                //  is_reusable: true
                                                            }
                                                        }
                                                    ).unwrap()
                                                }
                                            }
                                        } else {
                                            serde_json::to_value(
                                                Text { text: text_final }
                                            ).unwrap()
                                        }
                                    }
                                };


                                let page = token.page.clone().unwrap();
                                let value: SendFBIG<Value> = SendFBIG::new(
                                    page,
                                    st.number.clone(),
                                    dat,
                                    st.app.clone(),
                                    token.page_token.clone().unwrap());



                                if map.contains_key("voltar") {
                                    vec.clear();
                                    vec.push(value);
                                } else {
                                    vec.push(value);
                                }
                            }

                            Ok(vec)
                        }
                        ChatDataType::ButtonMidia(midia) => {
                            todo!();
                        }
                        ChatDataType::ButtonText(butto) => {
                            todo!();
                        }
                    };

                    Ok(i.unwrap())
                }
                Err(e) => { Err(e) }
            };
            match g {
                Ok(v) => {
                    let send = SendMessage::new(key);
                    send.send_instagram(v).await;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        "page" => {}
        _ => {

            println!("BOT WHATSAPP {} <> {}",&st.st, &st.app);



            let g = match db.get_chat(&st.st, &st.app).await {
                Ok(c) => {
                    let i = match c {
                        ChatDataType::Null => {

                            println!("null");

                            Err("null".to_string())
                        }
                        ChatDataType::Text(text) => {

                            println!("{:?}",text);

                            let mut vec = Vec::new();


                            for button in text {
                                let result = if button.midia {
                                    match button.type_midia {
                                        TypeMidia::AUDIO => {
                                            serde_json::to_value(
                                                MidiaType {
                                                    type_field: "audio".to_string(),
                                                    url: button.type_field,
                                                    filename: None,
                                                }
                                            ).unwrap()
                                        }
                                        TypeMidia::NULL => { todo!() }
                                        TypeMidia::IMAGE => {
                                            serde_json::to_value(
                                                ImageMidia {
                                                    type_field: "image".to_string(),
                                                    original_url: button.type_field.clone(),
                                                    preview_url: button.type_field,
                                                    caption: button.data.body.text,
                                                }
                                            ).unwrap()
                                        }
                                        TypeMidia::DOCUMENT => { todo!() }
                                        TypeMidia::VIDEO => {
                                            serde_json::to_value(
                                                ContentMD {
                                                    type_field: "video".to_string(),
                                                    url: button.type_field,
                                                    caption: "".to_string(),
                                                }
                                            ).unwrap()
                                        }
                                    }
                                } else {
                                    serde_json::to_value(
                                        MessageText { type_field: "text".to_string(), text: button.data.body.text }
                                    ).unwrap()
                                };

                                let value: SendWP<Value> = SendWP::new(
                                    st.app.as_str(),
                                    st.number.as_str(), get_number_app(st.app.as_str()),
                                    result);

                                vec.push(value);
                            }



                            Ok(vec)
                        }
                        ChatDataType::List(list) => {
                            let mut vec = Vec::new();

                            for bo in list {



                                let bot = bo.data;


                                let mut gb: Vec<GlobalButton> = bot.button_menu.iter().map(|c| {
                                    send_list_wp::GlobalButton {
                                        type_field: "text".to_string(),
                                        title: c.title.to_string(),
                                    }
                                }).collect();

                                let mut it: Vec<Item> = bot.payload.iter().enumerate().map(|(i, v)| {
                                    send_list_wp::Item {
                                        title: v.title.to_string(),
                                        subtitle: v.title.to_string(),
                                        options: v.itens.iter().enumerate().map(|(e, c)| send_list_wp::Optio {
                                            type_field: c.type_field.to_string(),
                                            title: c.title.to_string(),
                                            description: description_list_1(e as i32, st.st.as_str()),
                                            postback_text: Some(format!("n{}", e as i32)),
                                        }).collect(),
                                    }
                                }).collect();

                                /*   for i in 0..bot.button_menu.len() {
                                       let item: &Item = it.get(i).expect("");
                                       let btn: &GlobalButton = gb.get(i).expect("");
                                   } */

                                let mut text_final = if map.contains_key("voltar") {
                                    "Em que posso lhe ajudar?".to_string()
                                } else {
                                    match map.get("nodedouser") {
                                        None => { bot.body }
                                        Some(x) => { bot.body.replace("nodedouser", x.as_str()) }
                                    }
                                };

                                let dat = {
                                    if bot.show.unwrap() {
                                        serde_json::to_value(send_list_wp::Message {
                                            type_field: bo.type_field.to_string(),
                                            title: "".to_string(),
                                            body: text_final,
                                            msgid: Option::None,
                                            global_buttons: gb.clone(),
                                            items: it.clone(),
                                        }).unwrap()
                                    } else {
                                        if bo.midia {
                                            match bo.type_midia {
                                                TypeMidia::AUDIO => {
                                                    serde_json::to_value(
                                                        MidiaType {
                                                            type_field: "audio".to_string(),
                                                            url: bo.type_field,
                                                            filename: None,
                                                        }
                                                    ).unwrap()
                                                }
                                                TypeMidia::NULL => { todo!() }
                                                TypeMidia::IMAGE => {
                                                    serde_json::to_value(
                                                        ImageMidia {
                                                            type_field: "image".to_string(),
                                                            original_url: bo.type_field.clone(),
                                                            preview_url: bo.type_field,
                                                            caption: text_final,
                                                        }
                                                    ).unwrap()
                                                }
                                                TypeMidia::DOCUMENT => { todo!() }
                                                TypeMidia::VIDEO => {
                                                    serde_json::to_value(
                                                        ContentMD {
                                                            type_field: "video".to_string(),
                                                            url: bo.type_field,
                                                            caption: "".to_string(),
                                                        }
                                                    ).unwrap()
                                                }
                                            }
                                        } else {
                                            serde_json::to_value(
                                                MessageText { type_field: "text".to_string(), text: text_final }
                                            ).unwrap()
                                        }
                                    }
                                };


                                let value: SendWP<Value> = SendWP::new(
                                    st.app.as_str(),
                                    st.number.as_str(), get_number_app(st.app.as_str()),
                                    dat);

                                if map.contains_key("voltar") {
                                    vec.clear();
                                    vec.push(value);
                                } else {
                                    vec.push(value);
                                }
                            }

                            Ok(vec)
                        }
                        ChatDataType::ButtonMidia(midia) => {
                            todo!();
                        }
                        ChatDataType::ButtonText(butto) => {
                            let mut vec = Vec::new();

                            for button in butto {
                                let dat = {
                                    if button.data.show.unwrap() {
                                        let chat: ButtonWP<ContentBT> = ButtonWP {
                                            type_field: button.type_field,
                                            msgid: button.data.msgid,
                                            content: ContentBT {
                                                type_field: button.data.content.type_field,
                                                header: button.data.content.header,
                                                text: button.data.content.text,
                                                caption: button.data.content.caption,
                                            },
                                            options: button.data.options.iter().map(|c: &OptionB| OptionBT { type_field: c.type_field.clone(), title: c.title.clone() }).collect(),
                                        };

                                        serde_json::to_value(&chat).unwrap()
                                    } else {
                                        if button.midia {
                                            match button.type_midia {
                                                TypeMidia::AUDIO => {
                                                    serde_json::to_value(
                                                        MidiaType {
                                                            type_field: "audio".to_string(),
                                                            url: button.type_field,
                                                            filename: None,
                                                        }
                                                    ).unwrap()
                                                }

                                                TypeMidia::NULL => { todo!() }
                                                TypeMidia::IMAGE => {
                                                    serde_json::to_value(
                                                        ImageMidia {
                                                            type_field: "image".to_string(),
                                                            original_url: button.type_field.clone(),
                                                            preview_url: button.type_field,
                                                            caption: button.data.content.text,
                                                        }
                                                    ).unwrap()
                                                }
                                                TypeMidia::DOCUMENT => { todo!() }
                                                TypeMidia::VIDEO => {
                                                    serde_json::to_value(
                                                        ContentMD {
                                                            type_field: "video".to_string(),
                                                            url: button.type_field,
                                                            caption: "".to_string(),
                                                        }
                                                    ).unwrap()
                                                }
                                            }
                                        } else {
                                            serde_json::to_value(
                                                MessageText { type_field: "text".to_string(), text: button.data.content.text }
                                            ).unwrap()
                                        }
                                    }
                                };


                                let value: SendWP<Value> = SendWP::new(
                                    st.app.as_str(),
                                    st.number.as_str(), get_number_app(st.app.as_str()),
                                    dat);

                                vec.push(value);
                            }


                            Ok(vec)
                        }
                    };

                    Ok(i.unwrap())
                }
                Err(e) => { Err(e) }
            };


            match g {
                Ok(v) => {
                    let send = SendMessage::new(key);


                    send.send(v).await;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }


    Ok("OK".to_string())

    /* let data = json_to_data().await;
     match data {
         Ok(root) => {
             let bot = &root.data;

             let mut gb: Vec<GlobalButton> = bot.button_menu.iter().map(|c| {
                 send_list_wp::GlobalButton {
                     type_field: "text".to_string(),
                     title: c.title.to_string(),
                 }
             }).collect();

             let mut it: Vec<Item> = bot.payload.iter().map(|v| {
                 send_list_wp::Item {
                     title: v.title.to_string(),
                     subtitle: v.title.to_string(),
                     options: v.itens.iter().map(|c| send_list_wp::Optio {
                         type_field: c.type_field.to_string(),
                         title: c.title.to_string(),
                         description: Default::default(),
                         postback_text: Default::default(),
                     }).collect(),
                 }
             }).collect();


             for i in 0..bot.button_menu.len() {
                 let item: &Item = it.get(i).expect("");
                 let btn: &GlobalButton = gb.get(i).expect("");
                 if i == 0 {
                     let message = send_list_wp::Message {
                         type_field: root.type_field.to_string(),
                         title: "Serviços".to_string(),
                         body: bot.body.to_string(),
                         msgid: Option::None,
                         global_buttons: vec![btn.clone()],
                         items: vec![item.clone()],
                     };

                     let g: SendWP<Message> = SendWP::new(
                         st.app.as_str(),
                         st.number.as_str(), "917384811114", message);


                     let key = std::env::var("KEY_API").unwrap();
                     let send = SendMessage::new(key);
                     match send.send(g).await {
                         Ok(c) => { println!("{:?}", c); }
                         Err(e) => {
                             println!("{:?}", e.to_string());
                             return Err(e.to_string());
                         }
                     }
                 } else {
                     let message = send_list_wp::Message {
                         type_field: root.type_field.to_string(),
                         title: "Serviços".to_string(),
                         body: "*Outros Serviços*".to_string(),
                         msgid: Some(format!("list{}", i).to_string()),
                         global_buttons: vec![btn.clone()],
                         items: vec![item.clone()],
                     };

                     let g: SendWP<Message> = SendWP::new(
                         st.app.as_str(),
                         st.number.as_str(), "917384811114", message);




                 }
             }


             Ok("enviados".to_string())
         }
         Err(e) => { Err(e) }
     }  */
}


pub async fn deza(val: &Value, db: &MongoDb<'_>) {
    let d = val.get("value").unwrap();
    let app = val.get("app").unwrap().as_str().unwrap();

    let mut i = d.get("drawflow").unwrap();


    i = i.get("Home").unwrap();
    i = i.get("data").unwrap();

    let map = i.as_object().unwrap();

    db.delete_chat(app).await.unwrap();
    for x in map {
        let v = x.1;
        let mut datas = v.get("data").unwrap();

        let ty = v.get("html").unwrap().as_str().unwrap();

        println!("{}", ty);

        let typ = {
            match ty {
                "nodeOption" | "nodeOption2" | "nodeOption3" => "quick_reply",
                "NodeText" => "text",
                _ => { "list" }
            }
        };

        let status = datas.get("status").unwrap().as_str().unwrap();
        let a = datas.get("a").unwrap().as_str().unwrap();

        if typ == "text" {
            let chat: Chat<TextMongo> = Chat {
                id: None,
                index: status.to_string(),
                app: app.to_string(),
                data: TextMongo { body: Body { type_field: "text".to_string(), text: a.to_string() } },
                type_field: typ.to_string(),
                midia: false,
                type_midia: TypeMidia::NULL,
            };
            db.set_chat(serde_json::to_value(chat).unwrap()).await.unwrap();
        }
        if typ == "quick_reply" {
            let select = match datas.get("select") {
                None => "text",
                Some(c) => c.as_str().unwrap()
            };


            let op: &Vec<Value> = datas.get("op").unwrap().as_array().unwrap();

            match select {
                "text" => {
                    let chat: Chat<TextButtons<ContentText>> = Chat {
                        id: None,
                        index: status.to_string(),
                        app: app.to_string(),
                        midia: false,
                        data: TextButtons {
                            type_field: "text".to_string(),
                            msgid: "qlo".to_string(),
                            content: ContentText {
                                type_field: "text".to_string(),
                                header: "Serviços".to_string(),
                                text: a.to_string(),
                                caption: "caption".to_string(),
                            },
                            options: op.iter().map(|c| {
                                let ax = datas.get(format!("a-{}", c.as_i64().unwrap())).unwrap().as_str().unwrap();

                                OptionB { type_field: "text".to_string(), title: ax.to_string() }
                            }).collect(),
                            show: None,
                        },
                        type_field: typ.to_string(),
                        type_midia: TypeMidia::NULL,
                    };
                    db.set_chat(serde_json::to_value(chat).unwrap()).await.unwrap();
                }
                _ => {}
            }
        }


        if typ == "list" {
            let mut it: Vec<crate::chat::structs::list_mongo::Payload> = Vec::new();
            for ii in 0..3 {
                let mut c = ii * 10;

                let mut iitens: Vec<Iten> = Vec::new();
                for ee in c..(c + 10) {
                    match datas.get(format!("l-{}", ee)) {
                        None => {}
                        Some(c) => {
                            let iten = Iten { type_field: "text".to_string(), title: String::from(c.as_str().unwrap()) };
                            iitens.push(iten)
                        }
                    }
                }


                let count = iitens.len();
                let pay = crate::chat::structs::list_mongo::Payload { title: "".to_string(), itens: iitens };
                if count > 0 {
                    it.push(pay)
                }
            }

            let mut bt: Vec<ButtonMenu> = Vec::new();
            for i in 0..it.len() {
                let b = ButtonMenu { title: format!("Escolhe uma opção") };
                bt.push(b)
            }

            let chat: Chat<ListMongo> = Chat {
                id: None,
                index: status.to_string(),
                app: app.to_string(),
                data: ListMongo { show: Some(false), body: a.to_string(), payload: it, button_menu: bt },
                type_field: typ.to_string(),
                midia: false,
                type_midia: TypeMidia::NULL,
            };
            db.set_chat(serde_json::to_value(chat).unwrap()).await.unwrap();
        }
    }
}

// Object {"status": String("1"), "doc": Bool(false), "op": Array [], "type": Number(1), "term": Bool(false), "typ": String("text"), "g": Number(0), "a": String(""), "val": Array []}
// Object {"status": String("1-1"), "doc": Bool(false), "op": Array [Number(1), Number(2)], "type": Number(2), "term": Bool(false), "typ": String("button"), "g": Number(0), "a": String(""), "val": Array []}
//