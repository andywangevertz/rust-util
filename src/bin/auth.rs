use www_authenticate::{WwwAuthenticate, DigestChallenge, Qop, Algorithm};

// Authorization: Digest username="3000",realm="asterisk",nonce="2649f879",uri="sips:3000@172.16.246.13:5061",response="de1ad9566dc8085fcef7a623f375b3ea2907e650a3f007de2bc836874451092c",algorithm=SHA-256

fn main() {
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
}
