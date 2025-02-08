use super::Command;
use crate::error::{ShellError, ShellResult};
use crate::shell::Shell;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

pub struct ExternalCommand {
    name: String,
}

impl ExternalCommand {
    pub fn new(name: &str) -> ShellResult<Self> {
        Ok(Self {
            name: name.to_string(),
        })
    }
}

impl Command for ExternalCommand {
    fn execute(&self, args: &[&str], _shell: &Shell) -> ShellResult {
        let output = match ProcessCommand::new(&self.name).args(args).spawn() {
            Ok(child) => child.wait_with_output()?,
            Err(_) => return Err(ShellError::Command(format!("{}: command not found", &self.name))),
        };

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(ShellError::CommandFailed {
                code: output.status.code().unwrap_or(-1),
                message: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &'static str {
        "External system command"
    }
}

pub fn find_executable_path(command: &str) -> Result<PathBuf, ShellError> {
    if command.contains('/') {
        let path = PathBuf::from(command);
        if path.exists() {
            return Ok(path);
        }
    } else {
        if let Ok(path) = std::env::var("PATH") {
            for dir in path.split(':') {
                let full_path = PathBuf::from(dir).join(command);
                if full_path.exists() {
                    return Ok(full_path);
                }
            }
        }
    }

    Err(ShellError::Command(format!(
        "{}: not found",
        command
    )))
}
