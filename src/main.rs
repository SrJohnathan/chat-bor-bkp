use std::collections::HashMap;


use dotenvy::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::{Header, Method};


use rocket::{Request, Response, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
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

            if threads_number.contains_key(new_job.number.as_str()) {
                let thread = threads_number.remove(new_job.number.as_str()).unwrap();
                thread.abort();
            }

            if threads_number_speed.contains_key(new_job_sp.number.as_str()) {
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

                    delele_status(&new_job.number, &new_job.app, &db).await.unwrap();

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


    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);


    tokio::spawn(async move {
        match connection().await {
            Ok(c) => {
                let _ = rocket::build()
                    .attach(cors.to_cors().unwrap())
                    .manage(c)
                    .manage(config)
                    .manage(channel.0)
                    //    .mount("/", rocket_cors::catch_all_options_routes())
                    .mount("/",
                           routes![
                            http::gupshup_controller::web_hook,
                            http::http_controller::get,
                            http::http_controller::insert,
                               http::http_controller::facebook_token,
                            http::insta_controller::webhook,
                               http::insta_controller::messaging_webhook,
                               http::siga_controller::send,
                               http::siga_controller::template,
                               http::siga_controller::read_system,
                               http::siga_controller::send_archive,
                            http::siga_controller::agente

                       ])
                    //   .mount("/public", FileServer::from(rocket::fs::relative!("static")))
                    .launch()
                    .await;
            }
            Err(e) => { println!("{}", e.kind.to_string()) }
        }
    }).await.expect("TODO: panic message");
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST,PUT,GET,PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}