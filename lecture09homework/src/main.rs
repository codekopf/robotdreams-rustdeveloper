mod message;
mod server;
mod client;

use std::io;
use server::Server;
use std::thread;
use std::time::Duration;
use crate::client::Client;
use rand::{distributions::Alphanumeric, Rng};


const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PORT: u32 = 11111;

// abuday notes:
// run program -> cargo run
fn main() {

    // Let's first create a server
    println!("Enter hostname for server:");
    let mut hostname = String::new();
    io::stdin().read_line(&mut hostname).unwrap();
    let hostname = if hostname.trim().is_empty() {
        DEFAULT_HOSTNAME.to_string()
    } else {
        hostname.trim().to_string()
    };

    println!("Enter port for server:");
    let mut port_str = String::new();
    io::stdin().read_line(&mut port_str).unwrap();
    let port: u32 = port_str.trim().parse().unwrap_or(DEFAULT_PORT);

    let server = Server::new(&hostname, port);
    println!("Server created");

    // Start the server in a new thread
    thread::spawn(move || {
        if let Err(e) = server.start() {
            eprintln!("Server error: {}", e);
        }
    });

    let mut clients = vec![];

    loop {
        create_new_client(&hostname, port, &mut clients);

        message_engine(&mut clients);

        thread::sleep(Duration::from_secs(5));
    }

    //  + finish homework points 5,6,7 - https://robot-dreams-rust.mag.wiki/9-network-io/index.html
}

fn message_engine(clients: &mut Vec<Client>) {
    for client in &clients {
        let random_message = generate_random_message();
        println!("{} : {} - {}", client.ip(), client.port(), random_message);

        if let Err(e) = client.send_message(&random_message) {
            eprintln!("Client error: {}", e);
        }
    }
}

fn create_new_client(hostname: &String, port: u32, clients: &mut Vec<Client>) {
    println!("Do you want to create a new client? (yes/no)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "yes" {
        let client = Client::new(&hostname, port);
        println!("Client created with IP: {} and Port: {}", client.ip(), client.port());
        clients.push(client);
    }
}

fn generate_random_message() -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    format!("Random Message: {}", rand_string)
}