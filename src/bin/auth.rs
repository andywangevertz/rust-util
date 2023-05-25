use www_authenticate::{WwwAuthenticate, DigestChallenge, Qop, Algorithm};

// Authorization: Digest username="3000",realm="asterisk",nonce="2649f879",uri="sips:3000@172.16.246.13:5061",response="de1ad9566dc8085fcef7a623f375b3ea2907e650a3f007de2bc836874451092c",algorithm=SHA-256

fn main() {
  let user = "3000";
  let realm = "asterisk";
  let pass = "6a4be639166840848ddd384f2997e21c";
  let method = "REGISTER";
  let uri = "sips:3000@172.16.246.13:5061";
  let nonce = "4408d899";

  let auth = WwwAuthenticate::new(
  	DigestChallenge { 
		realm: Some("asterisk".into()),
		qop: Some(vec![Qop::Auth, Qop::AuthInt]),
		algorithm: Some(Algorithm::Sha256),
		nonce: Some("2649f879".into()),
		opaque: Some("opaque-dfdf".into()),
		domain: None,
		stale: None,
		userhash: None,
	});
   println!("{:#?}", auth);

  let sip_digest = util::sipdigest_sha(user, realm, pass, method, uri, nonce);
  println!("sip digest: {}", sip_digest);
  assert_eq!(sip_digest, "67949006ffbfa0bbc086247e4692390414198431e1c6b99406104532c1292f7b");


  let sip_digest = util::sipdigest_md5(user, realm, pass, method, uri, nonce);
  println!("sip digest: {}", sip_digest);
}
