//use std::io::{self, prelude::*};
//use std::fs::File;

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
// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
pub fn randomstr(len: usize) -> String {
  use rand::Rng;
  const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789)(*&^%$#@!~";
  let password_len: usize = len;
  let mut rng = rand::thread_rng();

  let password: String = (0..password_len)
      .map(|_| {
          let idx = rng.gen_range(0..CHARSET.len());
          CHARSET[idx] as char
      })
      .collect();
  password
}
pub fn initdigest(jcfg: &mut json::JsonValue) {
  // jcfg.contains() -- for array
  // jcfg.has_key()  -- for obj
  if !jcfg.has_key("method") {
    jcfg["method"] = "REGISTER".into();
  }
  if !jcfg.has_key("uri") {
    let sipx = if jcfg["transport"] == "TLS" { "sips" } else { "sip" };
    jcfg["uri"] =  format!("{}:{}@{}:{}", sipx, jcfg["user"], jcfg["server"], jcfg["port"]).into();
  }
  if !jcfg.has_key("nonce") {
    jcfg["nonce"] = randomstr(8).into();
  }
}
// server, port, user, password, realm, algo (MD5 SHA-256), transport (UDP TCP TLS)
pub fn sipdigest(jcfg: &mut json::JsonValue) -> String {
  initdigest(jcfg);
  
  let user = &jcfg["user"].to_string();
  let ream = &jcfg["realm"].to_string();
  let pass = &jcfg["password"].to_string();
  let method = &jcfg["method"].to_string();
  let uri = &jcfg["uri"].to_string();
  let nonce = &jcfg["nonce"].to_string();
  //println!("got jcfg[user]: {}, method: {}, uri: {}, nonce: {}", user, method, uri, nonce);
  
  if jcfg["algo"] == "SHA-256" {
    sipdigest_sha(user, ream, pass, method, uri, nonce)
  } else {
    sipdigest_md5(user, ream, pass, method, uri, nonce)
  }

}
