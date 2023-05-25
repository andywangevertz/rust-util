/*
REGISTER sips:3000@172.16.246.13:5061 SIP/2.0
Via: SIP/2.0/TLS 172.16.246.223;branch=z9hG4bK8f0d08d43354b5033
Max-Forwards: 64
From: <sips:3000@172.16.246.13:5061>;tag=bcef239ce8
To: <sips:3000@172.16.246.13:5061>
Call-ID: 2f0d3923df0dee27
CSeq: 1136739930 REGISTER
Allow: INVITE, ACK, CANCEL, BYE, NOTIFY, OPTIONS
Authorization: Digest username="3000",realm="asterisk",nonce="1fbab7fd",uri="sips:3000@172.16.246.13:5061",response="7a83fdc91ae05ad528204
a02ae3bafa8fed31651eefa41028bff50b2e398baa0",algorithm=SHA-256
Contact: <sips:3000@172.16.246.223:5061>;expires=60
User-Agent: Evertz UXP-TRS4K-2U/V300B20230516-71467002
Content-Length: 0
*/

// WWW-Authenticate: Digest algorithm=SHA-256, realm="asterisk", nonce="1fbab7fd", stale=true
use http::*;

fn main() {
   println!("hello, world!"); 
   
   let uri = "sips:3000@172.16.246.13:5061" ;
   let req = Request::builder()
      .method("REGISTER")
      .uri(uri)
      .version(Version::SIP_2)
      .header("Via", "SIP/2.0/TLS 172.16.246.223;branch=z9hG4k123456")
      .header("From",format!("<{}>;tag=bcef239c", uri))
      .header("To", format!("<{}>", uri))
      .header("Call-ID", "2f0d39")
      .header("CSeq", format!("{} {}", 120093938, "REGISTER"))
      .header("Allow", "INVITE, ACK, CANCEL, BYE, NOTIFY, OPTIONS")
      //.header("Authorization", format!("Digest {}", digest_str(user, realm, nonce, uri, response, algo)))
      .header("User-Agent", format!("Evertz {}/{}", "UXP-TRS4K-2U", "V300B20230516-71467002"))
      .header("Content-Length", "0")
      .body(())
      .unwrap();
  println!("{:#?}", req); 
  println!("{}", format!("{:?}", req));
}
