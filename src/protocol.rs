use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::TcpStream;

#[repr(u8)]
#[derive(Eq, PartialEq)]
pub enum MessageType {
    Empty = 0x00,
    CalculationStart = 0x01,
    CalculationEnd = 0x02,
    CalculationNumber = 0x03,
    CalculationSubmission = 0x04,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Message {
    pub msg_type: MessageType,
    payload_length: u8,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(msg_type: MessageType, payload: Vec<u8>) -> Self {
        Message {
            msg_type,
            payload_length: payload.len() as u8,
            payload,
        }
    }
}

pub fn send_message(stream: &mut TcpStream, msg: Message) {
    // write buffer
    let mut buffer: [u8; 1024] = [0; 1024];
    let payload = &msg.payload;
    let len = payload.len();

    buffer[0] = msg.msg_type as u8;
    buffer[1] = msg.payload_length;
    if len > 2 {
        buffer[2..2 + len].copy_from_slice(payload);
    }

    let total_len = 2 + len;

    let result = stream.write_all(&buffer[..total_len]);
    match result {
        Ok(r) => {
            println!("Message sent.")
        }
        Err(e) => {
            eprintln!("Send Message error: {e}")
        }
    }
}

pub fn recv_message(stream: &mut TcpStream) -> Option<Message> {
    // buffers
    let mut header_buffer: [u8; 2] = [0; 2];
    if let Err(e) = stream.read_exact(&mut header_buffer) {
        eprintln!("Failed to read header: {e}");
        return None;
    }

    let msg_type = match header_buffer[0] {
        0x01 => MessageType::CalculationStart,
        0x02 => MessageType::CalculationEnd,
        0x03 => MessageType::CalculationNumber,
        0x04 => MessageType::CalculationSubmission,
        _ => MessageType::Empty,
    };

    let payload_len = header_buffer[1] as usize;
    let mut payload_buffer: [u8; 255] = [0; 255];

    if payload_len > 0 {
        if payload_len > 255 {
            eprintln!("Payload exceeds allowed length (255).");
            return None;
        }

        if let Err(e) = stream.read_exact(&mut payload_buffer[..payload_len]) {
            eprintln!("Failed to read payload: {e}");
            return None;
        }
    }

    let payload: Vec<u8> = payload_buffer[..payload_len].to_vec();

    println!("Payload read.");

    Some(Message::new(msg_type, payload))
}
