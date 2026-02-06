use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Cannot bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Log connection
                println!("Connected");
                let source = stream.peer_addr().unwrap();
                let ip = source.ip();
                let s = source.port();
                println!("{ip}:{s}");

                let thread_join_handle = thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed, {e}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Send to client
    let source_port = stream.peer_addr().unwrap().port();
    let mut write_buffer: [u8; 1024] = [0; 1024];
    write_buffer[0] = 0b0000_0011;
    write_buffer[1] = 0b0000_1100;
    write_buffer[2] = 0b0011_0000;
    write_buffer[3] = 0b1100_0000;
    stream.write(&write_buffer).unwrap();

    let mut read_buffer: [u8; 4] = [0; 4];

    stream.read_exact(&mut read_buffer).unwrap();

    for c in read_buffer {
        println!("{c}");
    }

    println!("Closing for {source_port}");
}
