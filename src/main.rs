#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        evaluate_input(input.trim());
    }
}

fn evaluate_input(input: &str) {
    match input {
        "exit 0" => std::process::exit(0),
        _ => println!("{}: command not found", input),
    }
}
