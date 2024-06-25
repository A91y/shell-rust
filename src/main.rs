#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        input.pop(); // Remove the newline character
        let tokens: Vec<&str> = input.split(" ").collect();
        match tokens[0].trim() {
            "exit" => {
                std::process::exit(tokens.get(1).unwrap_or(&"0").parse::<i32>().unwrap());
            }
            "echo" => {
                println!("{}", tokens[1..].join(" "));
            }
            _ => {
                println!("{}: command not found", tokens[0].trim());
            }
        }
        input.clear()
    }
}
