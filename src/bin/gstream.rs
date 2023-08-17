use std::io::{self, Read, Write};
use std::net::{TcpStream, UdpSocket};

trait Stream: Read + Write {}

impl Stream for TcpStream {}
//impl Stream for UdpStream {}

fn create_stream() -> Box<dyn Stream> {
    // Example implementation: Create and return a TcpStream
    let tcp_stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to TCP server");
    Box::new(tcp_stream)

    // Example implementation: Create and return a UdpStream
    /*
    let udp_socket = UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind UDP socket");
    let udp_stream = UdpStream::new(udp_socket).expect("Failed to create UDP stream");
    Box::new(udp_stream)
    */
}

fn main() -> io::Result<()> {
    let mut stream = create_stream();

    // Example usage: Pass the created stream to other functions for processing
    process_stream(&mut stream)?;
    // process again
    process_stream(&mut stream)?;

    Ok(())
}

fn process_stream(stream: &mut Box<dyn Stream>) -> io::Result<()> {
    // Example usage: Read from the stream and print the received data
    let mut response = vec![0; 1024];
    let n = stream.read(&mut response)?;
    println!("Received: {}", String::from_utf8_lossy(&response[..n]));

    Ok(())
}

