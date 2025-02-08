use crate::error::{ShellError, ShellResult};
use crate::shell::Shell;
use std::io::{self};

use crate::output::OutputManager;

pub struct Repl {
    shell: Shell,
    output: OutputManager,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            shell: Shell::new(),
            output: OutputManager::new(),
        }
    }

    pub fn run(&mut self) -> ShellResult {
        loop {
            if let Err(e) = self.run_single_command() {
                match e {
                    ShellError::Exit(code) => std::process::exit(code),
                    _ => {
                        let _ = self.output.write_stderr(&format!("{}\n", e));
                    }
                }
            }
        }
    }

    fn run_single_command(&mut self) -> ShellResult {
        self.output.write_stdout("$ ")?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            return Ok(());
        }

        self.evaluate_input(input)
    }

    fn evaluate_input(&mut self, input: &str) -> ShellResult {
        let tokens = tokenize(input)?;
        let (cmd_tokens, stdout_redir, stderr_redir) = extract_redirections(tokens)?;

        if cmd_tokens.is_empty() {
            return Err(ShellError::Command("No command provided".into()));
        }

        if let Some((ref file, append)) = stdout_redir {
            self.output.redirect_stdout(file, append)?;
        }
        if let Some((ref file, append)) = stderr_redir {
            self.output.redirect_stderr(file, append)?;
        }

        let (command, args) = cmd_tokens
            .split_first()
            .ok_or_else(|| ShellError::Command("No command provided".into()))?;

        let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let result = self
            .shell
            .evaluate_command(command, &args, &mut self.output);

        self.output = OutputManager::new();

        result
    }
}

fn extract_redirections(
    tokens: Vec<String>,
) -> Result<(Vec<String>, Option<(String, bool)>, Option<(String, bool)>), ShellError> {
    let mut cmd_tokens = Vec::new();
    let mut stdout_redir = None;
    let mut stderr_redir = None;
    let mut i = 0;

    while i < tokens.len() {
        let token = &tokens[i];
        if token == ">"
            || token == "1>"
            || token == ">>"
            || token == "1>>"
            || token == "2>"
            || token == "2>>"
        {
            if i + 1 >= tokens.len() {
                return Err(ShellError::Command(
                    "Missing filename for redirection operator".into(),
                ));
            }
            let file = tokens[i + 1].clone();
            let append = token.contains(">>");
            if token.starts_with("2") {
                stderr_redir = Some((file, append));
            } else {
                stdout_redir = Some((file, append));
            }
            i += 2;
        } else {
            cmd_tokens.push(token.clone());
            i += 1;
        }
    }

    Ok((cmd_tokens, stdout_redir, stderr_redir))
}

fn tokenize(input: &str) -> Result<Vec<String>, ShellError> {
    #[derive(PartialEq)]
    enum State {
        Normal,
        InSingleQuote,
        InDoubleQuote,
    }

    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut state = State::Normal;
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        match state {
            State::Normal => {
                if ch.is_whitespace() {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                } else if ch == '\\' {
                    // Look ahead to process escape sequences.
                    if let Some(next_char) = chars.next() {
                        if next_char == '\n' {
                            // Backslash-newline: skip both (line continuation)
                            continue;
                        } else {
                            current_token.push(next_char);
                        }
                    } else {
                        // Trailing backslash -- push it literally.
                        current_token.push(ch);
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
                } else if ch == '\\' {
                    // In double quotes, backslash escapes only \, $, ", or newline.
                    if let Some(next_char) = chars.next() {
                        if next_char == '\\'
                            || next_char == '$'
                            || next_char == '"'
                            || next_char == '\n'
                        {
                            if next_char == '\n' {
                                // Line continuation within double quotes; skip both.
                                continue;
                            } else {
                                current_token.push(next_char);
                            }
                        } else {
                            // For any other character, keep the backslash literally.
                            current_token.push('\\');
                            current_token.push(next_char);
                        }
                    } else {
                        // Trailing backslash: push it literally.
                        current_token.push('\\');
                    }
                } else {
                    current_token.push(ch);
                }
            }
        }
    }

    if state != State::Normal {
        return Err(ShellError::Command("Unclosed quote in input".into()));
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    Ok(tokens)
}
