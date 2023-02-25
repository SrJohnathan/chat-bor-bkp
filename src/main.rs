use std::error::Error;
use dotenvy::dotenv;
use mongodb::Database;
use rocket::fs::{FileServer, relative};
use rocket::routes;
use tokio;
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


    match connection().await {
        Ok(c) => {

            let _ = rocket::build()
                .manage(c)
                .mount("/",
                       routes![
                            http::gupshup_controller::web_hook,
                            http::http_controller::get,
                            http::http_controller::insert

                       ])
                .mount("/public",FileServer::from(relative!("static")))
                .launch()
                .await;

        }
        Err(e) => {  println!("{}",  e.kind.to_string() )}
    }


}
