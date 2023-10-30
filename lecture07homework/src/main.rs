mod csv;

use std::env;
use std::error::Error;
use std::io;
use std::io::ErrorKind::InvalidInput;

// cargo run <action>
// <action>: lowercase, uppercase, no-spaces, slugify, sentence, unicode
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("lowercase") => handle_result(to_lowercase(&args)),
        Some("uppercase") => handle_result(to_uppercase(&args)),
        Some("no-spaces") => handle_result(no_spaces(&args)),
        Some("slugify") => handle_result(slugify(&args)),
        Some("sentence") => handle_result(turn_to_sentence(&args)),
        Some("unicode") => handle_result(turn_to_boolean(&args)),
        Some("csv") => handle_result(turn_to_csv(&args)),
        _ => display_invalid_action(),
    }
}

fn display_insufficient_input_length() {
    println!("Insufficient input length!");
}

fn handle_result(result: Result<String, Box<dyn Error>>) {
    match result {
        Ok(str) => println!("{}", str),
        Err(e) => eprintln!("Error: {}", e)
    }
}


fn to_lowercase(args: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let input = validate_user_input(&args);
    Ok(input?.to_lowercase())
}

fn to_uppercase(args: &Vec<String>) -> Result<String, Box<dyn Error>>  {
    let input = validate_user_input(&args);
    Ok(input?.to_uppercase())
}

fn no_spaces(args: &Vec<String>) -> Result<String, Box<dyn Error>>  {
    let input = validate_user_input(&args);
    Ok(input?.replace(' ', ""))
}

fn slugify(args: &Vec<String>) -> Result<String, Box<dyn Error>>  {
    let input = validate_user_input(&args);
    Ok(slug::slugify(input?))
}

fn turn_to_sentence(args: &Vec<String>) -> Result<String, Box<dyn Error>>  {
    let input = validate_user_input(&args);
    // let mut chars = input?.chars();
    let unwrapped_input = input?;
    let mut chars = unwrapped_input.chars();
    let first_char = chars.next().unwrap().to_uppercase().collect::<String>();
    Ok(format!("{}{}.", first_char, chars.collect::<String>()))
}

fn turn_to_boolean(args: &Vec<String>) -> Result<String, Box<dyn Error>>  {
    let input = validate_user_input(&args);
    let codes: Vec<u32> = input?.chars().map(|c| c as u32).collect();
    let single_line: String = codes.iter().map(|c| format!("{}", c)).collect::<Vec<String>>().join("");
    Ok(single_line)
}

fn turn_to_csv(args: &Vec<String>) -> Result<String, Box<dyn Error>>  {
    let input = validate_user_input(&args);
    // let mut chars = input?.chars();
    let unwrapped_input = input?;
    // TODO abuday - can't use line literal in terminal, instead using X as line separator
    let lines: Vec<&str> = unwrapped_input.split("X").collect();

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

fn validate_user_input(args: &Vec<String>) -> Result<String, Box<dyn Error>> {
    if args.len() < 2 {
        display_insufficient_arguments_message();
        return Err(Box::new(io::Error::new(InvalidInput, "Insufficient argument length!")));
    }

    let input = read_user_input();
    if input.is_empty() {
        display_insufficient_input_length();
        return Err(Box::new(io::Error::new(InvalidInput, "Insufficient input length!")));
    }

    Ok(input)
}

fn display_invalid_action()  {
    eprintln!("Invalid action! Use lowercase, uppercase, no-spaces, slugify, sentence or unicode.");
}

fn display_insufficient_arguments_message() {
    eprintln!("Usage: <program> <action>");
    eprintln!("Actions: lowercase, uppercase, no-spaces, slugify, sentence, unicode");
}

fn read_user_input() -> String {
    let mut input = String::new();
    format!("Enter the text:");
    io::stdin().read_line(&mut input).expect("Failed to read the input");
    input.trim().to_string()
}
