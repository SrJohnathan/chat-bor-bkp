use rocket::http::{ Status};
use rocket::serde::json::Json;
use rocket::{State, post, get,Request};


use rocket::request::{ FromRequest, Outcome};
use crate::chat::db_mongo::MongoDb;



use rocket::serde::Deserialize;
use crate::chat::ChatWP;
use crate::chat::models_instagram::ReceiverInstagram;

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


#[post("/instagram/chatbot",format = "application/json", data = "<task>")]
pub async fn webhook(db:MongoDb<'_>,config: &State<Config>, task: Json<ReceiverInstagram>) -> Result<String, Status> {
        println!("{}", serde_json::to_string(&task.0).unwrap());

// entity.messaging[0].sender.id.as_str()
    let f =  task.0;
    let entity  = &f.entry[0];
    let mut chat = ChatWP::new(entity.id.as_str(),f.object.as_str());
    if  f.object.eq(&"instagram".to_string()) {


        match  chat.run(&db).await {
            Ok(c) => {




            }
            Err(e) => { println!("erro {}",e)}
        }

    }else {


    }


    Ok("".to_string())
}


#[get("/instagram/chatbot")]
pub async fn messaging_webhook(config: &State<Config>, param: QueryParams) -> Result<String, Status> {
        if param.hub_mode == "subscribe".to_string() && param.hub_verify_token == config.verify_token {
            println!("WEBHOOK_VERIFIED");
            Ok(param.hub_challenge)
        } else {
            Err(Status::Forbidden)
        }


//  EAAFWay9FdBMBAFpfuiZBrY51aOVq9bD5ZBzB0OJz7LVcbk7XkT2fwZA9ZAuZB8IP0jivbilzvnbkvviCcq1XY0Y7cwlOFk0PMJ9ScplxIhlLWZBaCI5OjumADyz8ZCopVpPnEhZCBLvFsDU784P3OyaelyK3fhev4j8j1CAvy672ftLRIXZCzuLNa

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

}