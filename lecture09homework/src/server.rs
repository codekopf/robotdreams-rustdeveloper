use std::net::{TcpListener, TcpStream};
use std::io::{self, Read};
use std::thread;
use serde_cbor::{self, de};
use crate::message;
use message::Message;

pub struct Server {
    hostname: String,
    port: u32,
}

impl Server {

    pub fn new(hostname: &str, port: u32) -> Self {
        Server {
            hostname: hostname.to_string(),
            port,
        }
    }

    pub fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.hostname, self.port))?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        Server::handle_client(stream);
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = Vec::new();
        match stream.read_to_end(&mut buffer) {
            Ok(_) => {
                match de::from_slice::<Message>(&buffer) {
                    Ok(message) => println!("Received: {:?}", message),
                    Err(_) => println!("Failed to deserialize message"),
                }
            }
            Err(e) => println!("Failed to read from client: {}", e),
        }
    }
}