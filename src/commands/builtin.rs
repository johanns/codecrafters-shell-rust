use super::external::find_executable_path;
use super::Command;
use crate::error::{ShellError, ShellResult};
use crate::shell::Shell;

pub struct CdCommand;
pub struct EchoCommand;
pub struct ExitCommand;
pub struct PwdCommand;
pub struct TypeCommand;
pub struct HelpCommand;

impl CdCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for CdCommand {
    fn execute(
        &self,
        args: &[&str],
        _shell: &Shell,
        _output: &mut crate::output::OutputManager,
    ) -> ShellResult {
        let new_dir = args
            .get(0)
            .ok_or_else(|| ShellError::Command("cd: missing parameter".into()))?;

        let target_dir = if *new_dir == "~" {
            std::env::var("HOME").map_err(|_| ShellError::Command("cd: HOME not set".into()))?
        } else {
            new_dir.to_string()
        };

        std::env::set_current_dir(&target_dir).map_err(|_| {
            ShellError::Command(format!("cd: {}: No such file or directory", target_dir))
        })?;
        Ok(())
    }

    fn name(&self) -> &str {
        "cd"
    }

    fn description(&self) -> &'static str {
        "Change the current directory"
    }
}

impl EchoCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for EchoCommand {
    fn execute(
        &self,
        args: &[&str],
        _shell: &Shell,
        output: &mut crate::output::OutputManager,
    ) -> ShellResult {
        output.write_stdout(&format!("{}\n", args.join(" ")))?;
        Ok(())
    }

    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &'static str {
        "Print arguments to standard output"
    }
}

impl ExitCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for ExitCommand {
    fn execute(
        &self,
        args: &[&str],
        _shell: &Shell,
        _output: &mut crate::output::OutputManager,
    ) -> ShellResult {
        match args {
            [] => Err(ShellError::Command("exit: missing parameter".into())),
            [code] => {
                let code = code
                    .parse::<i32>()
                    .map_err(|_| ShellError::Command("exit: invalid parameter".into()))?;
                Err(ShellError::Exit(code))
            }
            _ => Err(ShellError::Command("exit: too many parameters".into())),
        }
    }

    fn name(&self) -> &str {
        "exit"
    }

    fn description(&self) -> &'static str {
        "Exit the shell with a status code"
    }
}

impl HelpCommand {
    pub fn new() -> Self {
        Self
    }

    fn print_help(&self, shell: &Shell, output: &mut crate::output::OutputManager) {
        let _ = output.write_stdout("Available commands:\n");
        let commands = shell.get_commands();
        let mut command_list: Vec<_> = commands.iter().collect();
        command_list.sort_by_key(|(name, _)| *name);

        for (name, cmd) in command_list {
            let _ = output.write_stdout(&format!("  {:<6} - {}\n", name, cmd.description()));
        }
    }
}

impl Command for HelpCommand {
    fn execute(
        &self,
        args: &[&str],
        shell: &Shell,
        output: &mut crate::output::OutputManager,
    ) -> ShellResult {
        if args.is_empty() {
            self.print_help(shell, output);
            return Ok(());
        }

        let command = args[0];
        if let Some(cmd) = shell.get_commands().get(command) {
            output.write_stdout(&format!("{} - {}\n", cmd.name(), cmd.description()))?;
        } else {
            return Err(ShellError::Command(format!("Unknown command: {}", command)));
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Display help information about available commands"
    }
}

impl PwdCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for PwdCommand {
    fn execute(
        &self,
        _args: &[&str],
        _shell: &Shell,
        output: &mut crate::output::OutputManager,
    ) -> ShellResult {
        let current_dir = std::env::current_dir()?;
        output.write_stdout(&format!("{}\n", current_dir.display()))?;
        
        Ok(())
    }

    fn name(&self) -> &str {
        "pwd"
    }

    fn description(&self) -> &'static str {
        "Print current working directory"
    }
}

impl TypeCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for TypeCommand {
    fn execute(
        &self,
        args: &[&str],
        shell: &Shell,
        output: &mut crate::output::OutputManager,
    ) -> ShellResult {
        if args.is_empty() {
            return Err(ShellError::Command("type: missing parameter".into()));
        }

        for arg in args {
            if shell.is_builtin(arg) {
                output.write_stdout(&format!("{} is a shell builtin\n", arg))?;
                continue;
            }

            match find_executable_path(arg) {
                Ok(path) => output.write_stdout(&format!("{} is {}\n", arg, path.display()))?,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "type"
    }

    fn description(&self) -> &'static str {
        "Display information about command type"
    }
}
