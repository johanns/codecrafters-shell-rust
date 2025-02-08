use crate::error::{ShellError, ShellResult};
use crate::shell::Shell;
use std::io::{self, Write};

pub struct Repl {
    shell: Shell,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            shell: Shell::new(),
        }
    }

    pub fn run(&mut self) -> ShellResult {
        loop {
            if let Err(e) = self.run_single_command() {
                match e {
                    ShellError::Exit(code) => std::process::exit(code),
                    _ => eprintln!("{}", e),
                }
            }
        }
    }

    fn run_single_command(&self) -> ShellResult {
        print!("$ ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            return Ok(());
        }

        self.evaluate_input(input)
    }

    fn evaluate_input(&self, input: &str) -> ShellResult {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let (command, args) = tokens
            .split_first()
            .ok_or_else(|| ShellError::Command("No command provided".into()))?;

        self.shell.evaluate_command(command, args)
    }
}
