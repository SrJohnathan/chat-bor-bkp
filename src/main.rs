use std::error::Error;
use dotenvy::dotenv;

use mongodb::Database;
use rocket::fs::{FileServer, relative};
use rocket::routes;
use tokio;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender,Receiver};

use crate::cofg::{JobWP, NewJob};

use crate::model::mongo::connection;

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
async fn main()  {
    dotenv().unwrap();
    let url = "DATABASE_URL";


    let mut channel:( Sender<String>,Receiver<String>) = mpsc::channel(100);

    tokio::spawn(async move {

        match channel.1.recv().await {
            Some(v) => {

                let new_job :NewJob = serde_json::from_str(&v).unwrap();

                tokio::spawn(async move {


                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;




                    println!("{:?}",new_job);



                });


            }
            None => { println!("the sender dropped"); }
        }
    });

        tokio::spawn(async move {
            match connection().await {
                Ok(c) => {
                    let _ = rocket::build()
                        .manage(c)
                        .manage(channel.0)
                        .mount("/",
                               routes![
                            http::gupshup_controller::web_hook,
                            http::http_controller::get,
                            http::http_controller::insert

                       ])
                      //  .mount("/public", FileServer::from(relative!("static")))
                        .launch()
                        .await;
                }
                Err(e) => { println!("{}", e.kind.to_string()) }
            }
        }).await.expect("TODO: panic message");




}
