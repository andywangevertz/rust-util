//use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    //let connector = TlsConnector::new().unwrap();

    //let stream = TcpStream::connect("google.com:443").unwrap();
    let mut stream = TcpStream::connect("192.168.223.25:80").unwrap();
    //let mut stream = connector.connect("google.com", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    /*
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
    */
    
    /*let mut res = String::new();
    stream.read_to_string(&mut res).unwrap();
    println!("{}", res);*/

    let mut line = [0; 10240];
    let mut res = String::new();
    loop {
       let result = stream.read(&mut line);
       match result {
         Ok(n) => if n > 0 { res.push_str(std::str::from_utf8(&line).unwrap()); } else { break; },
         _ => break,
       }
    }
    println!("{}", res);
}
