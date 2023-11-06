mod message;
mod server;
mod client;

use std::env;
use message::Message;
use server::Server;
use crate::client::Client;

const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PORT: u32 = 11111;

// abuday notes:
// run program -> cargo run <address> <port>
fn main() {
    let args: Vec<String> = env::args().collect();

    let default_hostname = DEFAULT_HOSTNAME.to_string();

    println!("Before arguments");

    let hostname = args.get(1).unwrap_or(&default_hostname);
    let port: u32 = args.get(2).unwrap_or(&DEFAULT_PORT.to_string()).parse().unwrap_or(DEFAULT_PORT);

    println!("After arguments");

    let server = Server::new(hostname, port);

    println!("Server created");

    if let Err(e) = server.start() {
        eprintln!("Server error: {}", e);
    }

    let client = Client::new(hostname, port);

    println!("Client created");

    // Here you should construct your message
    let message = Message::new("Some kind of title", "The body of the message");

    println!("Message created");

    if let Err(e) = client.send_message(&message) {
        eprintln!("Client error: {}", e);
    }

    println!("The end!");

    // TODO abuday
    //  + add an option to add a pool of clients
    //  + make infinite loop and
    //  + finish homework points 5,6,7 - https://robot-dreams-rust.mag.wiki/9-network-io/index.html
}