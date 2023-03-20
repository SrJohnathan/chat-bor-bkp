


use std::collections::HashMap;


use dotenvy::dotenv;



use rocket::routes;
use serde_json::Value;
use tokio;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::task::JoinHandle;
use crate::chat::send_list_wp::{MessageText, SendWP};

use crate::cofg::{get_number_app, NewJob};
use crate::http::insta_controller::Config;
use crate::http::models::SendMessage;

use crate::model::mongo::{connection, delele_status};

//mongo  stw  l1sLXHUz01OACdof

/*

use mongodb::{bson::doc, options::ClientOptions, Client};
#[tokio::main]
    async fn main() -> mongodb::error::Result<()> {
        let client_options = ClientOptions::parse(
            "mongodb+srv://stw:<password>@chat-wp.pmlgafg.mongodb.net/?retryWrites=true&w=majority",
        )
        .await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("testDB");
        Ok(())
    }

*/





mod model;
pub mod schema;
mod http;
pub mod cofg;
pub mod chat;


#[tokio::main]
async fn main() {
    dotenv().unwrap();
   // let url = "DATABASE_URL";


    let mut channel: (Sender<String>, Receiver<String>) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut threads_number: HashMap<String, JoinHandle<_>> = HashMap::new();
        let mut threads_number_speed: HashMap<String, JoinHandle<_>> = HashMap::new();

        while let Some(v) = channel.1.recv().await {


            let new_job: NewJob = serde_json::from_str(&v).unwrap();
            let new_job_sp: NewJob = serde_json::from_str(&v).unwrap();

            if  threads_number.contains_key(new_job.number.as_str()   ) {
                let thread = threads_number.remove(new_job.number.as_str()).unwrap();
                thread.abort();

            }

            if  threads_number_speed.contains_key(new_job_sp.number.as_str()   ) {
                let thread = threads_number_speed.remove(new_job_sp.number.as_str()).unwrap();
                thread.abort();

            }

            if new_job.etapa.as_str() != "exit" {
                threads_number.insert(new_job.number.clone(), tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
                    let key = std::env::var("KEY_API").unwrap();
                    let message = SendMessage::new(key);

                    let value: SendWP<Value> = SendWP::new(
                        new_job.app.as_str(),
                        new_job.number.as_str(), get_number_app(new_job.app.as_str()),
                        serde_json::to_value(
                            MessageText { type_field: "text".to_string(), text: "Por falta de resposta vamos encerrar este atendimento, pode sempre iniciar uma nova conversa enviando qualquer mensagem, obrigado!".to_string() }
                        ).unwrap());


                        let db = connection().await.unwrap();

                        delele_status(&new_job.number,&new_job.app,&db).await.unwrap();

                    let mut vec = Vec::new();
                    vec.push(value);
                    message.send(vec).await;
                }));
            }
            if new_job_sp.etapa.as_str() != "exit" {
                threads_number_speed.insert(new_job_sp.number.clone(), tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
                    let key = std::env::var("KEY_API").unwrap();
                    let message = SendMessage::new(key);

                    let value: SendWP<Value> = SendWP::new(
                        new_job_sp.app.as_str(),
                        new_job_sp.number.as_str(), get_number_app(new_job_sp.app.as_str()),
                        serde_json::to_value(
                            MessageText { type_field: "text".to_string(), text: "Escolha uma opção por favor🙏".to_string() }
                        ).unwrap());

                    let mut vec = Vec::new();
                    vec.push(value);
                    message.send(vec).await;
                }));
            }
        }

    });

    let config = Config { verify_token: "95699569".to_string() };


    tokio::spawn(async move {
        match connection().await {
            Ok(c) => {
                let _ = rocket::build()
                    .manage(c)
                    .manage(config)
                    .manage(channel.0)
                    .mount("/",
                           routes![
                            http::gupshup_controller::web_hook,
                            http::http_controller::get,
                            http::http_controller::insert,
                            http::insta_controller::web_hook,
                               http::insta_controller::messaging_webhook

                       ])
                     // .mount("/public", FileServer::from(relative!("static")))
                    .launch()
                    .await;
            }
            Err(e) => { println!("{}", e.kind.to_string()) }
        }
    }).await.expect("TODO: panic message");
}
