use std::io::{Read, Write};
use std::net::TcpStream;

pub fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");
    println!("Connected to server");

    let mut read_buffer: [u8; 4] = [0; 4];

    stream.read_exact(&mut read_buffer).unwrap();

    let sum: u8 = read_buffer.iter().sum();

    let write_buffer: [u8; 4] = [sum, 0, 0, 0];

    let mut str = String::new();
    let _ = std::io::stdin().read_line(&mut str);

    stream.write(&write_buffer).unwrap();
}
