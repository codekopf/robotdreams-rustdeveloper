use std::net::TcpStream;
use std::io::{self, Write};
use serde_cbor::ser;
use crate::message;
use message::Message;

pub struct Client {
    hostname: String,
    port: u32,
}

impl Client {

    pub fn new(hostname: &str, port: u32) -> Self {
        Client {
            hostname: hostname.to_string(),
            port,
        }
    }

    pub fn send_message(&self, message: &Message) -> io::Result<()> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.hostname, self.port))?;
        println!("Stream");
        let message_bytes = ser::to_vec(&message).expect("Serialization failed");
        stream.write_all(&message_bytes)?;
        Ok(())
    }

}