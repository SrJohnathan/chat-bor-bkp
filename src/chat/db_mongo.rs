use futures::TryStreamExt;
use mongodb::{bson, Client, Database};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{ClientOptions, UpdateOptions};
use regex::Regex;
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use serde_json::Value;
use crate::chat::factory_msg_send_text::{factory_text, TypeMidia};

use crate::chat::structs::{Chat, ChatDataType, ClientKeyBot};
use crate::chat::structs::list_mongo::ListMongo;
use crate::chat::structs::status::Status;
use crate::chat::structs::text_buttons::{ContentMedia, ContentText, TextButtons};
use crate::chat::structs::text_mongo::{Body, TextMongo};
use crate::http::models::{BotClient, FacebookToken, SendData};


pub struct MongoDb<'r>(pub &'r Database);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MongoDb<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let res = request.guard::<&State<Database>>().await;
        res.map(|c| MongoDb(c))
    }
}

impl<'r> MongoDb<'r> {
    pub async fn get_status(&self, number: &String, app: &String) -> Result<Status, String> {
        let filter = doc! { "number": number.as_str() , "app": app.as_str() };
        let typed_collection = self.0.collection::<Status>("status");
        let f = typed_collection.find_one(filter, None).await.unwrap();
        match f {
            None => { Err("Status Vazio".to_string()) }
            Some(s) => { Ok(s) }
        }
    }

    pub async fn get_status_all(&self, app: &String) -> Result<Vec<Status>, mongodb::error::Error> {
        let mut bots = Vec::new();
        let filter = doc! { "app": app.as_str() };
        let typed_collection = self.0.collection::<Status>("status");
        let mut f = typed_collection.find(None, None).await.unwrap();

        while let Some(dob) = f.try_next().await? {
            bots.push(dob);
        }
        Ok(bots)


    }

    pub async fn get_token_facebook(&self) -> Result<FacebookToken, String> {
        let typed_collection = self.0.collection::<FacebookToken>("token");
        let f = typed_collection.find_one( None,None).await.unwrap();
        match f {
            None => { Err("token facebook Vazio".to_string()) }
            Some(s) => { Ok(s) }
        }
    }

    pub async fn insert_or_update_facebook_token(&self, token: &FacebookToken) -> Result<bool, String> {
        let typed_collection = self.0.collection::<FacebookToken>("token");
        let filter = doc! {"_id": token.id.clone()};
        let update = doc! {"$set": bson::to_document(&token).unwrap()};
        let options = UpdateOptions::builder().upsert(true).build();
        let result = typed_collection.update_one(filter, update, options).await;

        match result {
            Ok(result) => {
                if result.matched_count > 0 {
                    Ok(true) // Updated existing token
                } else {
                    let f = typed_collection.insert_one(token, None).await;
                    match f {
                        Ok(_) => Ok(false), // Inserted new token
                        Err(_) => Err(String::from("error ao criar o token facebook")),
                    }
                }
            }
            Err(_e) => Err(String::from("error ao atualizar o token facebook")),
        }
    }

    pub async fn update_status(&self, st: &Status) -> Result<bool, mongodb::error::Error> {
        let filter = doc! { "number": st.number.as_str() , "app": st.app.as_str() };
        let bso = bson::to_bson(st).unwrap();
        let b = bso.as_document().unwrap();

        let typed_collection = self.0.collection::<Status>("status");
        let f = typed_collection.update_one(filter, doc! {"$set": b}, None).await;
        match f {
            Ok(v) => Ok(v.modified_count > 0),
            Err(err) => Err(err)
        }
    }

    pub async fn delele_status(&self, st: &Status) -> Result<bool, mongodb::error::Error> {
        let filter = doc! { "number": st.number.as_str() , "app": st.app.as_str() };
        let bso = bson::to_bson(st).unwrap();
        let b = bso.as_document().unwrap();

        let typed_collection = self.0.collection::<Status>("status");
        let f = typed_collection.delete_many(filter, None).await;
        match f {
            Ok(v) => Ok(v.deleted_count > 0),
            Err(err) => Err(err)
        }
    }

    pub async fn get_chat(&self, number: &String, app: &String) -> Result<ChatDataType, String> {

        let filter = doc! { "index": number.as_str(),"app": app.as_str()};


        let typed_collection = self.0.collection::<Chat<Value>>("chat");

        let f =  typed_collection.find_one(filter, None).await.map(|t| { t }).map_err(|e| {
            println!( "{}",e.to_string())
        }).unwrap();


        match f {
            None => { Err("Status Vazio".to_string()) }

            Some(s) => {

                match s.type_field.as_str() {
                    "text" => {


                        let mut vec = Vec::new();

                        let value: TextMongo = serde_json::from_value(s.data).unwrap();



                        let tmp: Vec<&str> = value.body.text.split("{|}").collect();

                        for v in tmp {
                            let v1 = v.replace("{|}", "");

                            let res = factory_text(v1);

                            let c: Chat<TextMongo> = Chat {
                                id: s.id,
                                index: s.index.clone(),
                                app: s.app.clone(),
                                data: TextMongo { body: Body{ type_field: "text".to_string(), text: res.0} },
                                type_field: if res.1 { res.2 } else { "text".to_string() },
                                midia: res.1,
                                type_midia: res.3,
                            };

                            vec.push(c);

                        }



                        Ok(ChatDataType::Text(vec))
                    }
                    "list" => {
                        let mut vec = Vec::new();

                        let  value: ListMongo = serde_json::from_value(s.data).unwrap();


                        let tmp: Vec<&str> = value.body.split("{|}").collect();

                        for v in tmp {
                            let  v1 = v.replace("{|}", "");

                            let res = factory_text(v1);

                            let vel = ListMongo {
                                body: res.0,
                                payload: value.payload.clone(),
                                button_menu: value.button_menu.clone(),
                                show: Some(res.4),
                            };


                            let c: Chat<ListMongo> = Chat {
                                id: s.id,
                                index: s.index.clone(),
                                app: s.app.clone(),
                                data: vel,
                                type_field: if res.1 { res.2 } else { "list".to_string() },
                                midia: res.1,
                                type_midia: res.3,
                            };


                            vec.push(c)
                        }


                        Ok(ChatDataType::List(vec))
                    }
                    "quick_reply" => {
                        let mut vec = Vec::new();
                        let value = s.data.get("type").unwrap();

                        if value.as_str().unwrap() == "text" {
                            let value: TextButtons<ContentText> = serde_json::from_value(s.data).unwrap();

                            let tmp: Vec<&str> = value.content.text.split("{|}").collect();

                            for v in tmp {
                                let  v1 = v.replace("{|}", "");


                                let res = factory_text(v1);

                                let c: Chat<TextButtons<ContentText>> = Chat {
                                    id: s.id,
                                    index: s.index.clone(),
                                    app: s.app.clone(),
                                    data: TextButtons {
                                        type_field: "text".to_string(),
                                        msgid: "4343".to_string(),
                                        content: ContentText {
                                            type_field: "text".to_string(),
                                            header: "".to_string(),
                                            text: res.0,
                                            caption: "".to_string(),
                                        },
                                        options: value.options.clone(),
                                        show: Some(res.4),
                                    },
                                    type_field: if res.1 { res.2 } else { "quick_reply".to_string() },
                                    midia: res.1,
                                    type_midia: res.3,
                                };

                                vec.push(c);
                            }

                            Ok(ChatDataType::ButtonText(vec))
                        } else {
                            let value: TextButtons<ContentMedia> = serde_json::from_value(s.data).unwrap();
                            let c: Chat<TextButtons<ContentMedia>> = Chat {
                                id: s.id,
                                index: s.index,
                                app: s.app,
                                data: value,
                                type_field: s.type_field,
                                midia: false,
                                type_midia: TypeMidia::NULL,
                            };
                            let mut vec = Vec::new();
                            vec.push(c);
                            Ok(ChatDataType::ButtonMidia(vec))
                        }
                    }

                    _ => { Ok(ChatDataType::Null) }
                }
            }
        }
    }
    pub async fn get_bot(&self) -> Result<Vec<Value>, mongodb::error::Error> {
        let mut bots = Vec::new();
        let typed_collection = self.0.collection::<Value>("bot");
        let mut f = typed_collection.find(None, None).await.unwrap();
        while let Some(dob) = f.try_next().await? {
            bots.push(dob);
        }

        Ok(bots)
    }
    pub async fn set_bot(&self, st: Value) -> Result<bool, String> {
        let bso = bson::to_bson(&st).unwrap();
        let b = bso.as_document().unwrap();

        let filter = doc! {  "app": st.get("app").unwrap().as_str().unwrap() };

        self.delete_bot(st.get("app").unwrap().as_str().unwrap()).await.unwrap();

        let typed_collection = self.0.collection::<Value>("bot");
        typed_collection.insert_one(st, None).await.unwrap();

        Ok(true)
    }
    pub async fn set_chat(&self, st: Value) -> Result<bool, String> {
        let bso = bson::to_bson(&st).unwrap();
        let b = bso.as_document().unwrap();

        let filter = doc! {  "app": st.get("app").unwrap().as_str().unwrap() };
        let typed_collection = self.0.collection::<Value>("chat");

        match typed_collection.insert_one(st, None).await {
            Ok(x) => { Ok(true) }
            Err(e) => { Err("error ao inserir".to_string()) }
        }

        /*  let f = typed_collection.insert_one(filter,doc! {"$set" : b }, None).await;
          match f {
              Ok(v) => {

                  if v.modified_count == 0 {
                      typed_collection.insert_one(st,None).await.unwrap();
                  }
                  Ok(true)

              },
              Err(err) => Err("".to_string())
          } */
    }
    pub async fn delete_chat(&self, app: &str) -> Result<bool, String> {
        let filter = doc! {  "app": app };
        let typed_collection = self.0.collection::<Value>("chat");


        match typed_collection.delete_many(filter, None).await {
            Ok(x) => { Ok(true) }
            Err(e) => { Err("error ao inserir".to_string()) }
        }

        /*  let f = typed_collection.insert_one(filter,doc! {"$set" : b }, None).await;
          match f {
              Ok(v) => {

                  if v.modified_count == 0 {
                      typed_collection.insert_one(st,None).await.unwrap();
                  }
                  Ok(true)

              },
              Err(err) => Err("".to_string())
          } */
    }
    pub async fn delete_bot(&self, app: &str) -> Result<bool, String> {
        let filter = doc! {  "app": app };
        let typed_collection = self.0.collection::<Value>("bot");


        match typed_collection.delete_many(filter, None).await {
            Ok(x) => { Ok(true) }
            Err(e) => { Err("error ao inserir".to_string()) }
        }

        /*  let f = typed_collection.insert_one(filter,doc! {"$set" : b }, None).await;
          match f {
              Ok(v) => {

                  if v.modified_count == 0 {
                      typed_collection.insert_one(st,None).await.unwrap();
                  }
                  Ok(true)

              },
              Err(err) => Err("".to_string())
          } */
    }




    pub async fn set_key_client(&self, value :SendData<Value> ) -> Result<bool, String> {

        let collection = self.0.collection::<SendData<Value>>("clienteBotKeys");
        let collection_bot = self.0.collection::<BotClient>("BotClient");


        let insert_result = collection.insert_one(&value, None).await;


        let bot = BotClient{
            id: None,
            name: value.name,
            phone: value.sid,
            show:true,
            app:None
        };

        let filter = doc! {"phone": &bot.phone};
        let existing_bot = collection_bot.find_one(filter, None).await.expect("TODO: panic message");

        if existing_bot.is_none() {
            // Bot does not exist, insert it

            collection_bot.insert_one(&bot, None).await.expect("TODO: panic message");

        }

        match insert_result {
            Ok(x) => {  Ok(true) }
            Err(w) =>  Err("Failed to insert the document".to_string())
        }

       /* let filter = doc! { "number":  value.sid.clone() };
        if let Some(existing_doc) = collection.find_one(filter.clone(), None).await.map_err(|e| e.to_string())? {
            // Document exists, update the keys array
            let existing_id: Option<ObjectId> = existing_doc.id;
            let existing_keys: Vec<String> = existing_doc.keys;

            let mut updated_keys = existing_keys.clone();
            updated_keys.extend_from_slice(&value.keys);

            let update = doc! {
                "$set": { "keys": updated_keys  , "show":true },
            };

            let update_result = collection.update_one(doc! { "_id": existing_id }, update, None).await.map_err(|e| e.to_string())?;

            // Check if the update was successful
            if update_result.modified_count > 0 {
                Ok(true)
            } else {
                Err("Failed to update the document".to_string())
            }
        } else {
            // Document does not exist, insert a new one


        }*/
    }

    pub async fn update_show_field(&self,number:String, show_value: bool) -> Result<(), String> {
        let collection = self.0.collection::<BotClient>("BotClient");

        let filter = doc! { "phone": number };

        let update = doc! {
            "$set": { "show": show_value },
        };

        // Execute a atualização
        let update_result = collection.update_one(filter, update, None).await.map_err(|e| e.to_string())?;

        if update_result.modified_count > 0 {
            Ok(())
        } else {
            Err("Falha ao atualizar o campo 'show'".to_string())
        }
    }

    pub async fn get_all_client_key_bots_by_app(&self, app_value: &str) -> Result<Vec<BotClient>, String> {
        let collection = self.0.collection::<BotClient>("BotClient");
        let filter = doc! { "phone": app_value };
        let cursor = collection.find(filter, None).await.map_err(|e| e.to_string())?;
        // Converta o cursor em um vetor de ClientKeyBot
        let client_key_bots: Vec<BotClient> = cursor
            .try_collect()
            .await
            .map_err(|e| e.to_string())?;

        Ok(client_key_bots)
    }

    pub async fn get_all_client_bot(&self, number: &str) -> Result<Vec<SendData<Value>>, String> {
        let collection = self.0.collection::<SendData<Value>>("clienteBotKeys");
        let filter = doc! { "sid": number };

        let cursor = collection.find(filter, None).await.map_err(|e| e.to_string())?;
        // Converta o cursor em um vetor de ClientKeyBot
        let client_key_bots: Vec<SendData<Value>> = cursor
            .try_collect()
            .await
            .map_err(|e| e.to_string())?;
        Ok(client_key_bots)
    }

}