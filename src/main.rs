#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

const BUILT_IN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];
fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    let path_env = std::env::var("PATH").unwrap();
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
            "type" => {
                handle_type(&path_env, &tokens[1].trim());
            }
            _ => {
                if tokens.len() == 1 && tokens[0].trim() == "" {
                    continue;
                }
                handle_command(&path_env, &tokens[0..].join(" "));
            }
        }
        input.clear()
    }
}

fn handle_type(path: &str, cmd: &str) {
    if BUILT_IN_COMMANDS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else {
        let dirs: Vec<&str> = path.split(":").collect();
        for dir in dirs {
            let path = format!("{}/{}", dir, cmd);
            if std::path::Path::new(&path).exists() {
                println!("{} is {}", cmd, path);
                return;
            }
        }
        println!("{}: not found", cmd);
    }
}

fn handle_command(path: &str, cmd: &str) {
    let dirs: Vec<&str> = path.split(":").collect();
    for dir in dirs {
        let command = cmd.split(" ").collect::<Vec<&str>>()[0];
        let arg = cmd.split(" ").collect::<Vec<&str>>()[1..].join(" ");
        let path = format!("{}/{}", dir, command);
        if std::path::Path::new(&path).exists() {
            let output;
            if arg == "" {
                output = Command::new(path).output().unwrap();
            } else {
                output = Command::new(path).arg(arg).output().unwrap();
            }
            println!("{}", String::from_utf8_lossy(&output.stdout).trim());
            return;
        }
    }
    println!("{}: command not found", cmd);
}
