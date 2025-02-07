#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{self, Write};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref COMMANDS: HashMap<&'static str, fn(&[&str])> = {
        let mut m = HashMap::new();
        m.insert("echo", cmd_echo as fn(&[&str]));
        m.insert("exit", cmd_exit as fn(&[&str]));
        m.insert("type", cmd_type as fn(&[&str]));
        m
    };
}

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

    if let Some(&function) = COMMANDS.get(command) {
        function(parameters);
    } else {
        println!("{}: command not found", input);
    }
}

//// Builtin Commands ////

fn cmd_echo(parameters: &[&str]) {
    println!("{}", parameters.join(" "));
}

fn cmd_exit(parameters: &[&str]) {
    if parameters.is_empty() {
        println!("exit: missing parameter");
    } else if parameters == ["0"] {
        std::process::exit(0);
    } else {
        println!("exit: invalid parameter");
    }
}

fn cmd_type(parameters: &[&str]) {
    if parameters.is_empty() {
        println!("type: missing parameter");
        return;
    }

    for param in parameters {
        if COMMANDS.contains_key(param) {
            println!("{} is a shell builtin", param);
            continue;
        }

        let path = match std::env::var("PATH") {
            Ok(p) => p,
            Err(_) => {
                println!("Could not read PATH environment variable");
                return;
            }
        };

        let mut found = false;
        for dir in path.split(':') {
            let full_path = std::path::Path::new(dir).join(param);
            if full_path.exists() {
                println!("{} is {}", param, full_path.display());
                found = true;
                break;
            }
        }

        if !found {
            println!("{}: not found", param);
        }
    }
}
