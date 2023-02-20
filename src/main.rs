use std::error::Error;
use rocket::routes;
use tokio;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "DATABASE_URL";

    let _ = rocket::build()
        .mount("/",
               routes![http::gupshup_controller::web_hook])
        .launch()
        .await;

    Ok(())
}
