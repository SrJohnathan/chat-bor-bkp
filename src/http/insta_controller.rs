use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{State, post,get};
use tokio::sync::mpsc::Sender;
use crate::chat::db_mongo::MongoDb;

#[post("/instagram/chatbot", format = "application/json", data = "<task>")]
pub async fn web_hook(db:MongoDb<'_>, job:&State<Sender<String>>,task: Json<serde_json::Value>)-> Status{

    println!("{:?}",task.0);

    Status::Ok
}

pub struct Config {
    pub(crate) verify_token: String,
}

#[get("/instagram/chatbot?<hub.mode>&<hub.verify_token>&<hub.challenge>")]
pub async fn messaging_webhook(config: &State<Config>, mode: Option<String>, token: Option<String>, challenge: Option<String>) -> Result<String, Status> {
    // Check if all required parameters are present
    let mode = mode.ok_or_else(|| Status::BadRequest)?;
    let token = token.ok_or_else(|| Status::BadRequest)?;
    let challenge = challenge.ok_or_else(|| Status::BadRequest)?;

    // Check if mode and token are correct
    if mode == "subscribe" && token == config.verify_token {
        // Respond with the challenge token from the request
        Ok(format!("WEBHOOK_VERIFIED\n{}", challenge))
    } else {
        // Respond with '403 Forbidden' if verify tokens do not match
        Err(Status::Forbidden)
    }
}