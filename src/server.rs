use crate::protocol::{recv_message, send_message, Message, MessageType};
use rand::rngs::ThreadRng;
use rand::RngExt;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Cannot bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Log connection
                println!("New connection:");
                let source = stream.peer_addr().unwrap();
                let ip = source.ip();
                let s = source.port();
                println!("{ip}:{s}");

                // connection's own thread
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {e}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Start calculation on client side
    send_message(
        &mut stream,
        Message::new(MessageType::CalculationStart, vec![]),
    );

    // send numbers
    let mut rng = ThreadRng::default();
    let count = rng.random_range(2..10);
    for _ in 0..count {
        let num = rng.random::<u32>();
        let payload = num.to_be_bytes().to_vec();

        send_message(
            &mut stream,
            Message::new(MessageType::CalculationNumber, payload),
        )
    }

    // finish calculation
    send_message(
        &mut stream,
        Message::new(MessageType::CalculationEnd, vec![]),
    );

    let result = recv_message(&mut stream);
    if let Some(msg) = result {
        match msg.msg_type {
            MessageType::CalculationSubmission => {
                let payload = msg.payload;
                let bytes: [u8; 8] = payload[..8].try_into().expect("Must contain 8 bytes");
                let submission = u64::from_be_bytes(bytes);

                println!("Given calculation results: {submission}");
            }
            _ => {
                eprintln!(
                    "Message type not implemented to be listened on the server: {}",
                    msg.msg_type
                )
            }
        }
    }
}
