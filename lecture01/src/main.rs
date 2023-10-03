use std::io;

fn main() {
    // println!("Hello, world!");

    println!("Enter your name:");

    let mut name: String = String::new(); // Rust can encourage from context

    // Better thing is to ignore it is to gracefully crash the program
    // let _ = io::stdin().read_line(&mut name); // ... "&mut" without giving ownership away

    io::stdin().read_line(&mut name).expect("Failed to read input line!");

    let name = name.trim();

    println!("Hello, {}", name);
}
