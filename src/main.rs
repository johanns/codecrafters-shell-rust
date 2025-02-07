use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("{0}")]
    Command(String),
    #[error("Exit code: {0}")]
    Exit(i32),
}

type ShellResult = Result<(), ShellError>;

#[derive(Clone)]
enum Command {
    Cd,
    Echo,
    Exit,
    Pwd,
    Type,
}

impl Command {
    fn execute(&self, shell: &Shell, args: &[&str]) -> ShellResult {
        match self {
            Command::Cd => {
                let new_dir = args
                    .get(0)
                    .ok_or(ShellError::Command("cd: missing parameter".into()))?;
                std::env::set_current_dir(new_dir).map_err(|_| {
                    ShellError::Command(format!("cd: {}: No such file or directory", new_dir))
                })?;
                Ok(())
            }
            Command::Echo => {
                println!("{}", args.join(" "));
                Ok(())
            }
            Command::Exit => match args {
                [] => Err(ShellError::Command("exit: missing parameter".into())),
                ["0"] => std::process::exit(0),
                _ => Err(ShellError::Command("exit: invalid parameter".into())),
            },
            Command::Pwd => {
                let current_dir = std::env::current_dir().map_err(ShellError::Io)?;
                println!("{}", current_dir.display());
                Ok(())
            }
            Command::Type => {
                if args.is_empty() {
                    return Err(ShellError::Command("type: missing parameter".into()));
                }

                for arg in args {
                    if shell.is_builtin(arg) {
                        println!("{} is a shell builtin", arg);
                        continue;
                    }

                    match find_executable_path(arg) {
                        Ok(path) => println!("{} is {}", arg, path.display()),
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
        }
    }
}

struct Shell {
    commands: HashMap<String, Command>,
}

impl Shell {
    fn new() -> Self {
        let mut commands = HashMap::new();
        commands.insert("cd".into(), Command::Cd);
        commands.insert("echo".into(), Command::Echo);
        commands.insert("exit".into(), Command::Exit);
        commands.insert("pwd".into(), Command::Pwd);
        commands.insert("type".into(), Command::Type);
        Self { commands }
    }

    fn is_builtin(&self, command: &str) -> bool {
        self.commands.contains_key(command)
    }

    fn run(&mut self) -> ShellResult {
        loop {
            print!("$ ");
            io::stdout().flush().map_err(ShellError::Io)?;

            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(ShellError::Io)?;

            if let Err(e) = self.evaluate_input(input.trim()) {
                match e {
                    ShellError::Exit(code) => std::process::exit(code),
                    _ => eprintln!("{}", e),
                }
            }
        }
    }

    fn evaluate_input(&self, input: &str) -> ShellResult {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.is_empty() {
            return Ok(());
        }

        let (command, args) = tokens
            .split_first()
            .ok_or_else(|| ShellError::Command("No command provided".into()))?;

        if let Some(cmd) = self.commands.get(*command) {
            cmd.execute(self, args)
        } else {
            self.execute_external(command, args)
        }
    }

    fn execute_external(&self, command: &str, args: &[&str]) -> ShellResult {
        let _ = find_executable_path(command)?;

        let output = std::process::Command::new(command)
            .args(args)
            .output()
            .map_err(ShellError::Io)?;

        io::stdout()
            .write_all(&output.stdout)
            .map_err(ShellError::Io)?;
        io::stderr()
            .write_all(&output.stderr)
            .map_err(ShellError::Io)?;

        Ok(())
    }
}

fn find_executable_path(command: &str) -> Result<PathBuf, ShellError> {
    let path = std::env::var("PATH").map_err(ShellError::Env)?;

    for dir in path.split(':') {
        let full_path = PathBuf::from(dir).join(command);
        if full_path.exists() {
            return Ok(full_path);
        }
    }

    Err(ShellError::Command(format!("{}: not found", command)))
}

fn main() -> ShellResult {
    let mut shell = Shell::new();
    shell.run()
}
