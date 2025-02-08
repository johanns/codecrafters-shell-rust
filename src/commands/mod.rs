use crate::error::ShellResult;
use crate::shell::Shell;
use std::collections::HashMap;

pub mod builtin;
pub mod external;

pub trait Command {
    fn execute(&self, args: &[&str], shell: &Shell) -> ShellResult;
    fn name(&self) -> &str;
    fn description(&self) -> &'static str;
}

pub fn create_command_map() -> HashMap<String, Box<dyn Command>> {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

    // Add built-in commands
    commands.insert("cd".into(), Box::new(builtin::CdCommand::new()));
    commands.insert("echo".into(), Box::new(builtin::EchoCommand::new()));
    commands.insert("exit".into(), Box::new(builtin::ExitCommand::new()));
    commands.insert("help".into(), Box::new(builtin::HelpCommand::new()));
    commands.insert("pwd".into(), Box::new(builtin::PwdCommand::new()));
    commands.insert("type".into(), Box::new(builtin::TypeCommand::new()));

    commands
}
