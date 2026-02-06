use crate::protocol::{recv_message, send_message, Message, MessageType};
use std::net::TcpStream;

pub fn client() {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");
    println!("Connected to server");

    handle_connection(stream);
}

fn handle_connection(mut stream: TcpStream) {
    let result = recv_message(&mut stream);
    if let Some(msg) = result {
        if msg.msg_type != MessageType::CalculationStart {
            return;
        }
    } else {
        return;
    }

    let mut sum: u64 = 0;
    loop {
        let result = recv_message(&mut stream);
        if let Some(msg) = result {
            match msg.msg_type {
                MessageType::CalculationNumber => {
                    let bytes: [u8; 4] = msg.payload[..4].try_into().expect("Not 4 sized");
                    let num: u32 = u32::from_be_bytes(bytes);
                    sum += num as u64;
                }
                MessageType::CalculationEnd => {
                    break;
                }
                _ => {}
            }
        }
    }

    let payload: Vec<u8> = sum.to_be_bytes().to_vec();
    send_message(
        &mut stream,
        Message::new(MessageType::CalculationSubmission, payload),
    );
}
