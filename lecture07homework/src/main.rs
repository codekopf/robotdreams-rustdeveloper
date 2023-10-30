mod csv;
mod command;

use std::error::Error;
use std::io;
use std::io::ErrorKind::InvalidInput;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use command::Command;
use std::str::FromStr;
use std::time::{Duration, Instant}; // TODO abuday - This is weird - I take Traits as intefaces (Java~Scala). Why do I need to add their scope here?
use std::fs;

// abuday notes:
// run program -> cargo run
// test CSV -> csv C:\DEV\robotdreams-rustdeveloper\lecture07homework\test.txt
fn main() {

    let (sender, receiver) = mpsc::channel();

    let input_thread = create_sender_thread(sender);

    let processing_thread = create_processing_thread(receiver);

    // Wait for threads to finish
    input_thread.join().unwrap();
    processing_thread.join().unwrap();
}

fn create_processing_thread(receiver: Receiver<(Command, String)>) -> JoinHandle<()> {
    thread::spawn(move || {
        // As I have no clue what I am doing, this is safe-fail switch to kill the infinite loop after 50 seconds
        let start_time = Instant::now();
        loop {
            if start_time.elapsed() > Duration::from_secs(50) {
                break;
            }

            let (command, input) = receiver.recv().unwrap();
            match command {
                Command::Lowercase => handle_result(to_lowercase(&input)),
                Command::Uppercase => handle_result(to_uppercase(&input)),
                Command::NoSpaces => handle_result(no_spaces(&input)),
                Command::Slugify => handle_result(slugify(&input)),
                Command::Sentence => handle_result(turn_to_sentence(&input)),
                Command::Unicode => handle_result(turn_to_boolean(&input)),
                Command::Csv => handle_result(turn_to_csv(&input)),
            }
        }
    })
}

fn create_sender_thread(sender: Sender<(Command, String)>) -> JoinHandle<()> {
    thread::spawn(move || {
        // As I have no clue what I am doing, this is safe-fail switch to kill the infinite loop after 50 seconds
        let start_time = Instant::now();
        loop {
            if start_time.elapsed() > Duration::from_secs(50) {
                break;
            }

            let input = read_user_input();
            if input.is_empty() {
                display_insufficient_input_length();
                continue;
            }

            if let Some((command_str, input_text)) = input.split_once(' ') {
                if let Ok(command) = Command::from_str(command_str) {
                    if command == Command::Csv {
                        let file_content = fs::read_to_string(input_text.trim());
                        match file_content {
                            Ok(content) => {
                                sender.send((command, content)).unwrap();
                            },
                            Err(e) => {
                                eprintln!("Failed to read the file: {}", e);
                            }
                        }
                    } else {
                        sender.send((command, input_text.to_string())).unwrap();
                    }
                } else {
                    display_invalid_action()
                }
            } else {
                display_insufficient_arguments_message();
                io::Error::new(InvalidInput, "Insufficient argument length!");
            }
        }
    })
}

fn display_insufficient_input_length() {
    println!("Insufficient input length!");
}

fn display_insufficient_arguments_message() {
    eprintln!("You need to write request in the form <command> <text>");
}

fn display_invalid_action()  {
    eprintln!("Invalid action! Use lowercase, uppercase, no-spaces, slugify, sentence or unicode.");
}

fn handle_result(result: Result<String, Box<dyn Error>>) {
    match result {
        Ok(str) => println!("{}", str),
        Err(e) => eprintln!("Error: {}", e)
    }
}

fn to_lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

fn to_uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

fn no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(' ', ""))
}

fn slugify(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(slug::slugify(input))
}

fn turn_to_sentence(input: &str) -> Result<String, Box<dyn Error>> {
    let mut chars = input.chars();
    let first_char = chars.next().unwrap().to_uppercase().collect::<String>();
    Ok(format!("{}{}.", first_char, chars.collect::<String>()))
}

fn turn_to_boolean(input: &str) -> Result<String, Box<dyn Error>> {
    let codes: Vec<u32> = input.chars().map(|c| c as u32).collect();
    let single_line: String = codes.iter().map(|c| format!("{}", c)).collect::<Vec<String>>().join("");
    Ok(single_line)
}

fn turn_to_csv(input: &str) -> Result<String, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Err(Box::new(io::Error::new(InvalidInput, "CSV data is empty!")));
    }

    let headers: Vec<String> = lines[0].split(',').map(|s| s.trim().to_string()).collect();
    let mut values = Vec::new();

    for line in &lines[1..] {
        let row: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        if row.len() == headers.len() {
            values.push(row);
        } else {
            return Err(Box::new(io::Error::new(InvalidInput, "CSV rows have inconsistent column counts!")));
        }
    }

    let csv_data = csv::Csv { headers, values };
    Ok(csv_data.to_string())
}

fn read_user_input() -> String {
    let mut input = String::new();
    println!("Enter the command (lowercase, uppercase, no-spaces, slugify, sentence, unicode, csv) followed by the text or path to CSV file:");
    io::stdin().read_line(&mut input).expect("Failed to read the input");
    input.trim().to_string()
}
