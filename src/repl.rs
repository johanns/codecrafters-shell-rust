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
        let tokens = self.tokenize(input)?;

        let (command, args) = tokens
            .split_first()
            .ok_or_else(|| ShellError::Command("No command provided".into()))?;

        let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        self.shell.evaluate_command(command, &args)
    }

    fn tokenize(&self, input: &str) -> Result<Vec<String>, ShellError> {
        #[derive(PartialEq)]
        enum State {
            Normal,
            InSingleQuote,
            InDoubleQuote,
        }

        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut state = State::Normal;

        for ch in input.chars() {
            match state {
                State::Normal => {
                    if ch.is_whitespace() {
                        if !current_token.is_empty() {
                            tokens.push(current_token);
                            current_token = String::new();
                        }
                    } else if ch == '\'' {
                        state = State::InSingleQuote;
                    } else if ch == '"' {
                        state = State::InDoubleQuote;
                    } else {
                        current_token.push(ch);
                    }
                }
                State::InSingleQuote => {
                    if ch == '\'' {
                        state = State::Normal;
                    } else {
                        current_token.push(ch);
                    }
                }
                State::InDoubleQuote => {
                    if ch == '"' {
                        state = State::Normal;
                    } else {
                        current_token.push(ch);
                    }
                }
            }
        }

        if state != State::Normal {
            return Err(ShellError::Command(
                "Unclosed quote in input".into(),
            ));
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        Ok(tokens)
    }
}
