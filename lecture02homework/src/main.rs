use std::env;
use std::io;

// cargo run <action>
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: <program> <action>");
        println!("Actions: lowercase, uppercase, no-spaces, slugify, sentence, unicode");
        return;
    }

    let action = &args[1];
    let mut input = String::new();

    println!("Enter the text:");
    io::stdin().read_line(&mut input).expect("Failed to read the input");
    let input = input.trim();

    if input.is_empty() {
        println!("Insufficient input length!")
    }

    match action.as_str() {
        "lowercase" => {
            println!("{}", input.to_lowercase());
        }
        "uppercase" => {
            println!("{}", input.to_uppercase());
        }
        "no-spaces" => {
            println!("{}", input.replace(' ', ""));
        }
        "slugify" => {
            println!("{}", slug::slugify(input));
        }
        "sentence" => {
            let mut chars = input.chars();
            let first_char = chars.next().unwrap().to_uppercase().collect::<String>();
            println!("{}{}.", first_char, chars.collect::<String>());
        }
        "unicode" => {
            let codes: Vec<u32> = input.chars().map(|c| c as u32).collect();
            for code in codes  {
                print!("{}", code);
            }
        }
        _ => {
            println!("Invalid action! Use lowercase, uppercase, no-spaces, slugify, sentence or unicode.");
        }
    }

}