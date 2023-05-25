use std::io::{self, prelude::*};
use std::fs::File;

pub fn parseconfig(cfg: &str) -> json::JsonValue {
  let contents = std::fs::read_to_string(cfg).expect("configuration file is missing!");
  let jobj = json::parse(&contents).expect("the configuration should be json format");
  jobj
}

pub fn sha256digest(str: &str) -> String
{
   String::from(sha256::digest(str))
}
pub fn md5digest(str: &str) -> String
{
    let md5str = format!("{:x}", md5::compute(str));
    String::from(md5str)
}
pub fn sipdigest_sha(user: &str, realm: &str, pass: &str, method: &str, uri: &str, nonce: &str) -> String
{
   
  let a1= format!("{}:{}:{}", user, realm, pass); //"3000:asterisk:6a4be639166840848ddd384f2997e21c";
  let a1hash = sha256::digest(a1);  //sha256_secret

  let a2 = format!("{}:{}", method, uri); //"REGISTER:sips:3000@172.16.246.13:5061";
  let a2hash = sha256::digest(a2);

  let response = format!("{}:{}:{}", a1hash, nonce, a2hash);
  let resphash = sha256::digest(response);

  String::from(resphash)
  
}
pub fn sipdigest_md5(user: &str, realm: &str, pass: &str, method: &str, uri: &str, nonce: &str) -> String
{
   
  let a1= format!("{}:{}:{}", user, realm, pass); //"3000:asterisk:6a4be639166840848ddd384f2997e21c";
  let a1hash = md5digest(&a1);  //sha256_secret

  let a2 = format!("{}:{}", method, uri); //"REGISTER:sips:3000@172.16.246.13:5061";
  let a2hash = md5digest(&a2);

  let response = format!("{}:{}:{}", a1hash, nonce, a2hash);
  let resphash = md5digest(&response);

  String::from(resphash)
  
}

pub fn sipdigest(jcfg: &json::JsonValue) {
  println!("got jcfg[user]: {}", jcfg["user"]);
}
