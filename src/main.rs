#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{self, Write};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref COMMANDS: HashMap<&'static str, fn(&[&str]) -> Result<(), String>> = {
        let mut m = HashMap::new();
        m.insert("echo", cmd_echo as fn(&[&str]) -> Result<(), String>);
        m.insert("exit", cmd_exit as fn(&[&str]) -> Result<(), String>);
        m.insert("type", cmd_type as fn(&[&str]) -> Result<(), String>);
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

    // Try built-in commands first
    if let Some(&function) = COMMANDS.get(command) {
        match function(parameters) {
            Ok(_) => return,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    }

    // Next, find and execute an executable with params
    match find_executable_path(command) {
        Ok(_) => {
            let mut cmd = std::process::Command::new(command);
            cmd.args(parameters);
            let output = cmd.output().unwrap();
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

//// Builtin Commands ////

fn cmd_echo(parameters: &[&str]) -> Result<(), String> {
    println!("{}", parameters.join(" "));
    Ok(())
}

fn cmd_exit(parameters: &[&str]) -> Result<(), String> {
    if parameters.is_empty() {
        return Err(format!("exit: missing parameter"));
    } else if parameters == ["0"] {
        std::process::exit(0);
    } else {
        return Err(format!("exit: invalid parameter"));
    }
}

fn cmd_type(parameters: &[&str]) -> Result<(), String> {
    if parameters.is_empty() {
        return Err(format!("type: missing parameter"));
    }

    for param in parameters {
        if COMMANDS.contains_key(param) {
            println!("{} is a shell builtin", param);
            continue;
        }

        match find_executable_path(param) {
            Ok(p) => println!("{} is {}", param, p),
            Err(e) => {
                return Err(e);
            }
        };
    }

    Ok(())
}

//// Helper Functions ////

fn find_executable_path(param: &str) -> Result<String, String> {
    let path = std::env::var("PATH")
        .map_err(|_| "Could not read PATH environment variable".to_string())?;

    for dir in path.split(':') {
        let full_path = std::path::Path::new(dir).join(param);
        if full_path.exists() {
            return Ok(full_path.display().to_string());
        }
    }

    Err(format!("{}: not found", param))
}
