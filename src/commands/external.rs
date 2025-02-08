use super::Command;
use crate::error::{ShellError, ShellResult};
use crate::shell::Shell;
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
    fn execute(
        &self,
        args: &[&str],
        _shell: &Shell,
        output: &mut crate::output::OutputManager,
    ) -> ShellResult {
        let child_output = match ProcessCommand::new(&self.name)
            .args(args)
            .output() {
            Ok(output) => output,
            Err(_) => {
                return Err(ShellError::Command(format!(
                    "{}: command not found",
                    &self.name
                )))
            }
        };

        output.write_stdout(&String::from_utf8_lossy(&child_output.stdout))?;
        output.write_stderr(&String::from_utf8_lossy(&child_output.stderr))?;

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

    Err(ShellError::Command(format!("{}: not found", command)))
}
