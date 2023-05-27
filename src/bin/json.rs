#![allow(dead_code, unused_variables)]

use std::io::{self, prelude::*};
use std::fs::File;
fn parseconfig1(cfg: &str) -> io::Result<()> 
{
  let mut f = File::open(cfg)?;
  //let mut buffer = Vec::new(); 
  //f.read_to_end(&mut buffer).unwrap();
  let mut buffer = String::new();
  f.read_to_string(&mut buffer)?;
  let jobj = json::parse(&buffer).unwrap();
  //println!(" jobj: {:#?}", jobj);
  
  // user, realm, nonce, uri, response, algo
  Ok(())
} 

fn parseconfig2(cfg: &str) -> json::JsonValue {
  let contents = std::fs::read_to_string(cfg).expect("configuration file is missing!");
  let jobj = json::parse(&contents).expect("the configuration should be json format");
  //println!("{} {} {} {}", jobj["server"], jobj["port"], jobj["user"], jobj["password"]);
  jobj
}

fn main() {
  //let ret = parseconfig1("3000.json");
  //let jobj = parseconfig2("3000.json"); 
  let mut jobj = util::parseconfig("3000.json"); 
  println!("{} {} {} {}", 
     jobj["server"], jobj["port"], jobj["user"], jobj["password"]);
  //util::initdigest(&mut jobj);
  println!("get digest: {}", util::sipdigest(&mut jobj));
}
