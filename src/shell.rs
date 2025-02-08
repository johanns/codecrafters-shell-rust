use crate::commands::{create_command_map, external::ExternalCommand, Command};
use crate::error::ShellResult;
use std::collections::HashMap;

pub struct Shell {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            commands: create_command_map(),
        }
    }

    pub fn is_builtin(&self, command: &str) -> bool {
        self.commands.contains_key(command)
    }

    pub fn get_commands(&self) -> &HashMap<String, Box<dyn Command>> {
        &self.commands
    }

    pub fn evaluate_command(&self, command: &str, args: &[&str], output: &mut crate::output::OutputManager) -> ShellResult {
        if let Some(cmd) = self.commands.get(command) {
            cmd.execute(args, self, output)
        } else {
            // Create and execute external command
            let external_cmd = ExternalCommand::new(command)?;
            external_cmd.execute(args, self, output)
        }
    }
}
