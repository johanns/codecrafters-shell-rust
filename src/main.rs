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

        // Evaluate the input
        evaluate_input(input.trim());
    }
}

fn evaluate_input(input: &str) {
    // Tokenize the input into commands and parameters
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.is_empty() {
        return;
    }

    let command = tokens[0];
    let parameters = &tokens[1..];

    match command {
        "echo" => println!("{}", parameters.join(" ")),
        "exit" if parameters == ["0"] => std::process::exit(0),
        _ => println!("{}: command not found", input),
    }
}
