use rocket::http::{RawStr, Status};
use rocket::serde::json::Json;
use rocket::{State, post, get,Request};
use rocket::form::{DataField, FromFormField, ValueField};
use rocket::http::uri::Query;

use rocket::request::{FromParam, FromRequest,FromSegments, Outcome};
use tokio::sync::mpsc::Sender;
use crate::chat::db_mongo::MongoDb;

#[post("/instagram/chatbot", format = "application/json", data = "<task>")]
pub async fn web_hook(db:MongoDb<'_>, job:&State<Sender<String>>,task: Json<serde_json::Value>)-> Status{

    println!("{:?}",task.0);

    Status::Ok
}

use rocket::serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct QueryParams {
    #[serde(rename = "hub.mode")]
    hub_mode: String,
    #[serde(rename = "hub.verify_token")]
    hub_verify_token: String,
    #[serde(rename = "hub.challenge")]
    hub_challenge: String,
}

impl std::str::FromStr for QueryParams {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut params = s.split('&').map(|kv| {
            let mut kv = kv.split('=');
            (kv.next().unwrap(), kv.next().unwrap_or("".as_ref()))
        }).collect::<Vec<_>>();

        let mode = params.iter().find(|(k, _)| *k == "hub.mode").map(|(_, v)| v.to_string()).ok_or_else(|| "Missing hub.mode".to_string())?;
        let token = params.iter().find(|(k, _)| *k == "hub.verify_token").map(|(_, v)| v.to_string()).ok_or_else(|| "Missing hub.verify_token".to_string())?;
        let challenge = params.iter().find(|(k, _)| *k == "hub.challenge").map(|(_, v)| v.to_string()).ok_or_else(|| "Missing hub.challenge".to_string())?;

        Ok(QueryParams { hub_mode: mode, hub_verify_token: token, hub_challenge: challenge })
    }
}


#[rocket::async_trait]
impl<'a> FromRequest<'a> for QueryParams {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {

       let q = request.uri().query().unwrap();
       let f:QueryParams =   q.parse().unwrap();

        Outcome::Success(f)

       // todo!()
    }
}




pub struct Config {
    pub verify_token: String,
}




#[get("/instagram/chatbot")]
pub async fn messaging_webhook(config: &State<Config>, param: QueryParams) -> Result<String, Status> {
    // Check if all required parameters are present








  /*  let mut query :QueryParams = c.parse().unwrap();

    let mode = query.hub_mode.ok_or_else(|| Status::BadRequest)?;
    let token =  query.hub_verify_token.ok_or_else(|| Status::BadRequest)?;
    let challenge =  query.hub_challenge.ok_or_else(|| Status::BadRequest)?;

    // Check if mode and token are correct
    if mode == "subscribe" && token == config.verify_token {
        // Respond with the challenge token from the request
        Ok(format!("WEBHOOK_VERIFIED\n{}", challenge))
    } else {
        // Respond with '403 Forbidden' if verify tokens do not match
        Err(Status::Forbidden)
    } */
    Err(Status::Forbidden)
}