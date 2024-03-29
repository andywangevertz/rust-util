use std::io::prelude::*;
//use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpStream};
//use std::time::Duration;
use std::io::BufReader;

// for TLS connection
use native_tls::{Certificate, TlsConnector, TlsStream};
use std::fs;
// https://stackoverflow.com/questions/68316251/rust-use-common-trait-type-for-different-streams-tcp-stream-tls-stream
pub trait IOReadWrite: Read + Write  {}
impl <T: Read + Write > IOReadWrite for T {}

use std::net::UdpSocket;

pub struct UdpStream {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl UdpStream {
    // passing the localip:port and remoteip:port
    pub fn init(local: &str, remote: &str) -> io::Result<Self> {
      let socket = UdpSocket::bind(local)?;
      socket.connect(remote)?;
      let buf = vec![0; 4096];
      Ok(Self {socket, buf })
    }
    // or creating outside and just passing the socket
    pub fn new(socket: UdpSocket) -> io::Result<Self> {
        let buf = vec![0; 4096]; // Set an appropriate buffer size
        Ok(Self { socket, buf })
    }
}

impl Read for UdpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let (n, _) = self.socket.recv_from(buf)?;
        Ok(n)
    }
}

impl Write for UdpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.socket.send(buf)?;
        Ok(n)
    }
    fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        let n = self.socket.send(buf)?;
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

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

pub fn request_to_string<T> (req: &http::Request<T>) -> String {
    let mut req_lines = format!("{} {} {:?}\r\n", req.method(), req.uri(), req.version());
    for (key, value) in req.headers().iter() {
         req_lines.push_str(&format!("{}: {}\r\n", key, value.to_str().unwrap()));
    }
    req_lines.push_str("\r\n");
    // followed by req.body()
    req_lines
}

pub fn read_stream(stream: &mut TcpStream) -> String {
    let mut line = [0; 1024];
    let mut res = String::new();
    loop {
       let result = stream.read(&mut line);
       match result {
         Ok(n) => if n > 0 { res.push_str(std::str::from_utf8(&line).unwrap()); } else { break; },
         _ => { println!("read from stream error!"); break; },
       }
    }
    res
}

pub fn vecs_to_lines(lines: & Vec<String>) -> String {
    let mut res = String::new();
    for line in lines.iter() {
       res.push_str(&format!("{}\r\n", line));   
    }
    res 
}

pub fn read_vecs<S>(stream: &mut S) -> Vec<String> 
where S: IOReadWrite,
{
    let buf_reader = BufReader::new(stream);
    let sip_resp: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect(); 
    sip_resp
}

pub fn read_lines<S>(stream: &mut S) -> String 
where S: IOReadWrite,
{
    let sip_resp = read_vecs(stream);
    vecs_to_lines(&sip_resp)
}

pub fn extract_string(input: &str, reg: &str) -> (String, String) {
   let re = regex::Regex::new(reg).unwrap();
   let caps = re.captures(input);
   if caps.is_none() {
      return (String::new(), String::new());
   } 
   let caps = caps.unwrap(); 
   let p1 = caps.get(1).map_or("", |m| m.as_str());
   let p2 = caps.get(2).map_or("", |m| m.as_str());
   (String::from(p1), String::from(p2))
}

// much like vecs_to_lines, but instead, extract useful information from it and return
// return neccessary (key,value) from response
pub fn parse_vecs(lines: & Vec<String>) -> (String, String, String) {
    let status = &lines[0];
    let mut method = String::new(); 
    let mut nonce = String::new();
    let mut via = String::new();
    for line in lines.iter() {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[0] {
	    "CSeq:" => method = String::from(parts[2]),
            "WWW-Authenticate:" => nonce = String::from(parts[4]),
             // Via: SIP/2.0/TCP 172.16.246.101;branch=z9hG4k123456;received=172.16.246.101;rport=52072
             "Via:" => via = String::from(parts[2]),
            _ => (),
        }
        //if parts[0].find("CSeq") { method = parts[2]; }
        //else if parts[0].find("WWW-Authenticate")
    }   
    if via.len() > 0 { 
       let (ip, port) = extract_string(&via, "received=(.*?);rport=(.*?)$");
    } 
 
    if status.find("401 Unauthorized").is_some() {
       // extract nonce: nonce="7d5a9102"
       let (part1, part2) = extract_string(&nonce, "\"(.*?)\"");
       (String::from("nonce"), part1, via)
    } else {
       (method, String::from(status), via)
    }
}

// create UDP/TCP/TLS stream based on the json config
pub fn init_stream(jcfg: & json::JsonValue) -> Box<dyn IOReadWrite> 
{
  let transport = jcfg["transport"].to_string();
  if transport == "TCP" { 
     let ipport = format!("{}:{}", jcfg["server"], jcfg["port"]);
     let stream = TcpStream::connect(ipport).expect("connect error!");
     Box::new(stream)

  } else if transport == "TLS" {
     let ipport  = format!("{}:{}", jcfg["server"], jcfg["port"]);
     let servername  = jcfg["server"].to_string();
     let connector: TlsConnector;
     if jcfg.has_key("certfile") {
        let certfile = jcfg["certfile"].to_string();
    	let cert_data = fs::read(certfile).expect("read certfile error!");
    	let cert = Certificate::from_pem(&cert_data).expect("convert to certificate error!");
    	connector = TlsConnector::builder()
              .add_root_certificate(cert)
              .build().expect("creae connector error!");
     } else {
    	connector = TlsConnector::builder()
              .build().expect("creae connector error!");
     }
     let stream = TcpStream::connect(ipport).expect("connect error!");
     let stream = connector.connect(&servername, stream).expect("connect to servername error!");
     Box::new(stream)

  } else {
     let local = format!("{}:{}", jcfg["local"], jcfg["port"]);
     let ipport = format!("{}:{}", jcfg["server"], jcfg["port"]);
     let stream = UdpStream::init(&local, &ipport).expect("udp init error!");
     Box::new(stream)

  } 

}
pub fn send_to_stream<S>(stream:&mut S, msg: &str) 
where S: IOReadWrite,
{
  stream.write_all(msg.as_bytes()).expect("write to stream error!");
}
pub fn read_from_stream<S>(stream:&mut S) -> Vec<String>
where S: IOReadWrite,
{
  read_vecs(stream)
}
// So at this stage, we could have one thread reading and one thread sending..

pub fn send_to_vecs(ipport: &str, msg: &str) -> (Box<dyn IOReadWrite>, Vec<String>) 
{
  let mut stream = TcpStream::connect(ipport).expect("connect error!");
  stream.write_all(msg.as_bytes()).expect("write to stream error!");
  let res = read_vecs(&mut stream);
  (Box::new(stream), res)
}

pub fn send_to_vecs_udp(local: &str, ipport: &str, msg: &str) -> (Box<dyn IOReadWrite>, Vec<String>) 
{
  let mut stream = UdpStream::init(local, ipport).expect("udp init error!");
  stream.write_all(msg.as_bytes()).expect("write to stream error!");
  let res = read_vecs(&mut stream);
  (Box::new(stream), res)
}

// certfile: certificate file for the server. chains file with multiple certs won't work 
// servername: server name in certificate. most likely the same as server's IP in SM case.
//     = so, ipport would be "172.16.246.13:5061" and serveranme is "172.16.246.13"
pub fn send_to_vecs_tls(ipport: &str, msg: &str, certfile: &str, servername: &str) -> (Box<dyn IOReadWrite>, Vec<String>) 
{
    let cert_data = fs::read(certfile).expect("read certfile error!");
    let cert = Certificate::from_pem(&cert_data).expect("convert to certificate error!");
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .build().expect("creae connector error!");
 
  let stream = TcpStream::connect(ipport).expect("connect error!");

  let mut stream = connector.connect(servername, stream).expect("connect to servername error!");

  stream.write_all(msg.as_bytes()).expect("write to stream error!");
  let res = read_vecs(&mut stream);
  (Box::new(stream), res)
}

// send one msg at a time and then disconnect
pub fn send_to(ipport: &str, msg: &str) -> (Box<dyn IOReadWrite>, String) 
{
  let mut stream = TcpStream::connect(ipport).expect("connect error!");
  //stream.set_nonblocking(true);
  //stream.set_read_timeout(Some(Duration::from_millis(2000)));
  stream.write_all(msg.as_bytes()).expect("write to stream error!");

  /* do not work... will block till EOF
  let mut res = String::new();  //vec![]; for read_to_end
  stream.read_to_string(&mut res).expect("read from stream error!");
  (stream, res)                 //String::from_utf8(res).unwrap())
  */
  //let res = read_stream(&mut stream);
  let res = read_lines(&mut stream);
  (Box::new(stream), res)
}

pub fn send_more<S>(stream: &mut S, msg: &str) -> String 
where S: IOReadWrite,
{
  stream.write_all(msg.as_bytes()).expect("write to stream error!");
  let res = read_lines(stream);
  res
}

