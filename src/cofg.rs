use std::sync::mpsc::{channel, Receiver, Sender};

pub const HOST_API_GUPSHUP:&str = "https://api.gupshup.io";
pub const MESSAGE_PATH_GUPSHUP:&str ="/sm/api/v1/msg";

pub const API_DEV:&str ="1wnuo9xzw0xlnavgtd0zjkqlwv7yci9x";
pub const API_PRODU:&str = "ku8gzeihiztucp71pog5xoipestl5abp";

pub fn get_number_app(app:&str) -> &str {

     match app {
         "WhatsAppSTWpt" => "351253930233",
         _ => "917834811114",
    }

}

pub fn get_app_app(app:&str) -> &str {

    match app {
        "WhatsAppSTWpt" => API_PRODU,
        _ => API_DEV,
    }

}

pub async fn aviso_whts(){

    let (sender, receiver): (Sender<bool>, Receiver<bool>) = channel();

}