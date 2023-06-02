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

fn regen_request(parsed: &mut json::JsonValue) -> String 
{
   let response = util::sipdigest(parsed);
  
   let uri = parsed["uri"].to_string();
   let user = parsed["user"].to_string();
   let realm = parsed["realm"].to_string();
   let nonce = parsed["nonce"].to_string();
   let algo  = parsed["algo"].to_string(); 
   let trans = parsed["transport"].to_string();
   let local= parsed["local"].to_string();
   let authheader = format!("Digest username=\"{}\",realm=\"{}\",nonce=\"{}\",uri=\"{}\",response=\"{}\",algorithm=\"{}\"",
       &user, &realm, &nonce, &uri, &response, &algo); 
   let req = Request::builder()
      .method("REGISTER")
      .uri(&uri)
      .version(Version::SIP_2)
      .header("Via", format!("SIP/2.0/{} {};branch=z9hG4k123456", &trans, &local))
      .header("From",format!("<{}>;tag=bcef239c", &uri))
      .header("To", format!("<{}>", &uri))
      .header("Call-ID", "2f0d39")
      .header("CSeq", format!("{} {}", 120093938, "REGISTER"))
      .header("Allow", "INVITE, ACK, CANCEL, BYE, NOTIFY, OPTIONS")
      .header("Authorization", authheader)
      .header("User-Agent", format!("Evertz {}/{}", "UXP-TRS4K-2U", "V300B20230516-71467002"))
      .header("Content-Length", "0")
      .body(())
      .unwrap();
  println!("{:#?}", req); 
  //println!("{}", format!("{:?}", req));
  //println!("onwire: \r\n{}", req.to_string());
  let request = util::request_to_string(&req);
  request
}

/* sip_evertz.conf for [extension] config
~ qualifyfreq=13000
+ nat=force_rport,comedia
+ transport=tcp,udp
*/

fn main() {
   println!("hello, world!"); 
   let mut parsed = json::parse(r#"
{  "server": "172.16.246.13",
   "port" : "5061",
   "user" : "3000",
   "password" : "7abc14aac22b3ffab8148aeb25f97f2e",
   "realm" : "asterisk",
   "algo": "MD5",
   "transport" : "TLS",
   "nonce" : "4408d899",
   "local" : "172.16.246.101"
}
"#).unwrap();

   let request = regen_request(&mut parsed);
   println!("onwire: \r\n{}", &request);

  //let (mut stream, ret) = util::send_to("192.168.223.12:5060", &request); 
  //let (stream, ret) = util::send_to("172.16.246.30:5060", &request); 
  let (mut stream, ret) = util::send_to_vecs_tls("172.16.246.13:5061", &request, "server.pem", "172.16.246.13"); 
  println!("get resp: {:?}", ret); 
  let (name, value, via) = util::parse_vecs(&ret);
  println!("name: {}, value: {}", name, value); 
  if name == "nonce" && value.len() > 0 {
     parsed["nonce"] = value.into();
     let request = regen_request(&mut parsed);
     println!("onwire: \r\n{}", &request);
     let ret = util::send_more(&mut stream, &request); 
     println!("get resp: {}", ret); 
  }

}
