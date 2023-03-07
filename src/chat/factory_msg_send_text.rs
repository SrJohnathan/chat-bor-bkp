use regex::Regex;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeMidia {
    NULL,
    IMAGE,
    DOCUMENT,
    VIDEO
}



pub fn factory_text(mut v1:String) -> (String, bool, String, TypeMidia, bool) {


        let mut type_midia = TypeMidia::NULL;
        let mut mi = false;
        let mut show_list = false;
        let mut url = String::from("type");
        let re = Regex::new(r"\{\{(.*?)\}\}").unwrap();
        let result = re.replace_all(v1.as_str(), |caps: &regex::Captures| {
            let name = &caps[1];
            match name {
                "name" => "joão",


                "type" => {
                    show_list = true;

                    ""
                }

                _ => {

                    let mut g: Vec<&str> = name.split("::").collect();
                    let qg: Vec<String> = g.iter().map(|x| x.replace("::", "")).collect();

                    match qg[0].as_str() {

                        "image" => { mi = true ; type_midia = TypeMidia::IMAGE; url  = qg[1].clone(); "" }
                        "video" => { mi = true ; type_midia = TypeMidia::VIDEO ; url  = qg[1].clone(); "" }
                        "document" => { mi = true ; type_midia = TypeMidia::DOCUMENT; url  = qg[1].clone(); "" }
                        &_ => todo!()
                    }

                },
            }
        });




    (result.to_string(),mi,url,type_midia ,show_list)

}