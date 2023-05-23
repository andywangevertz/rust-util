#[allow(unused_doc_comments)]
use sha256::digest;

fn main() {
  /// sha256(user:realm:password)
  //sha256_secret=2c39cacddecfcd2a0f0b087a23647fb65e9a5b532361d60a8583cd116d37c553
  //sha256_secret= echo -n "3000:asterisk:6a4be639166840848ddd384f2997e21c" |sha256sum
  let user = "3000";
  let realm = "asterisk";
  let password = "6a4be639166840848ddd384f2997e21c";
  let a1= format!("{}:{}:{}", user, realm, password); //"3000:asterisk:6a4be639166840848ddd384f2997e21c";
  let a1hash = digest(a1);  //sha256_secret
  println!("a1hash {}", a1hash);
  assert_eq!(a1hash, "2c39cacddecfcd2a0f0b087a23647fb65e9a5b532361d60a8583cd116d37c553");

  /// sha256(method:uri)
  //H2 = echo -n "REGISTER:sips:3000@172.16.246.13:5061" | sha256sum
  //a9f474b7e0df63314eff35b777a9f5a71a243f855131a6506fad8697ef75ed86
  let method = "REGISTER";
  let uri = "sips:3000@172.16.246.13:5061";
  let a2 = format!("{}:{}", method, uri); //"REGISTER:sips:3000@172.16.246.13:5061";
  let a2hash = digest(a2);
  println!("a2hash {}", a2hash);
  assert_eq!(a2hash, "a9f474b7e0df63314eff35b777a9f5a71a243f855131a6506fad8697ef75ed86");

  /// sha256(a1hash:nonce:a2hash)  ==> (sha256_secret:nonce:sha256(method:uri))
  //response = echo -n "2c39cacddecfcd2a0f0b087a23647fb65e9a5b532361d60a8583cd116d37c553:4408d899:a9f474b7e0df63314eff35b777a9f5a71a243f855131a6506fad8697ef75ed86" | sha256sum
  //==> [ 67949006ffbfa0bbc086247e4692390414198431e1c6b99406104532c1292f7b ]
  let nonce = "4408d899";
  let response = format!("{}:{}:{}", a1hash, nonce, a2hash);
  let resphash = digest(response);
  println!("response {}", resphash);
  assert_eq!(resphash, "67949006ffbfa0bbc086247e4692390414198431e1c6b99406104532c1292f7b");
}
