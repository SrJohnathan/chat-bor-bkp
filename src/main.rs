use std::error::Error;
use rocket::routes;
use tokio;


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
