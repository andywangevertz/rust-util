// from ChatGPT
// https://chat.openai.com/share/0ea463f9-141f-495a-b4d5-ac066076a3ab
use std::io::{self, Read, Write};

/* - the following implementation is moved to lib.rs
use std::net::UdpSocket;

struct UdpStream {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl UdpStream {
    // passing the localip:port and remoteip:port
    fn init(local: &str, remote: &str) -> io::Result<Self> {
      let socket = UdpSocket::bind(local)?;
      socket.connect(remote)?;
      let buf = vec![0; 4096];
      Ok(Self {socket, buf }) 
    }
    // or creating outside and just passing the socket
    fn new(socket: UdpSocket) -> io::Result<Self> {
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
*/


/* please paste the following to the stdin and then press Enter!

REGISTER sips:3000@172.16.246.13:5061 SIP/2.0
Via: SIP/2.0/TLS 172.16.246.223;branch=z9hG4bK8f0d08d43354b5033
Max-Forwards: 64
From: <sips:3000@172.16.246.13:5061>;tag=bcef239ce8
To: <sips:3000@172.16.246.13:5061>
Call-ID: 2f0d3923df0dee27
CSeq: 1136739930 REGISTER
Allow: INVITE, ACK, CANCEL, BYE, NOTIFY, OPTIONS
Authorization: Digest username="3000",realm="asterisk",nonce="1fbab7fd",uri="sips:3000@172.16.246.13:5061",response="7a83fdc91ae05ad528204a02ae3bafa8fed31651eefa41028bff50b2e398baa0",algorithm=SHA-256
Contact: <sips:3000@172.16.246.223:5061>;expires=60
User-Agent: Evertz UXP-TRS4K-2U/V300B20230516-71467002
Content-Length: 0


*/

fn main() -> io::Result<()> {
    //let socket = UdpSocket::bind("172.25.252.156:5060")?;
    //socket.connect("192.168.223.12:5060")?;
    //let mut stream = UdpStream::new(socket)?;
    let mut stream = util::UdpStream::init("172.25.252.156:5060", "192.168.223.12:5060")?;

    // Example usage: Read from stdin and write to the UDP socket
    //let mut input = String::new();
    //io::stdin().read_line(&mut input)?;
    //stream.write(input.as_bytes())?;
	
    /*
    let lines = io::stdin().lines();
    for line in lines {
      stream.write(line.unwrap().as_bytes())?;
    }*/

    let mut input = String::new();
    let lines = io::stdin().lines();
    for line in lines {
      let mystr = line.unwrap();
      let empty = mystr.len() == 0;
      //println!("get line: {}", mystr.len());
      input.push_str(&mystr);
      input.push_str("\r\n");
      if empty { break; }
    }
    stream.write(input.as_bytes())?;

    let mut response = vec![0; 1024];
    let n = stream.read(&mut response)?;
    println!("Received: {}", String::from_utf8_lossy(&response[..n]));

    Ok(())
}

