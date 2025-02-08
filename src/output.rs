use std::fs::OpenOptions;
use std::io::Write;

use crate::error::ShellResult;

pub struct OutputManager {
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}

impl OutputManager {
    pub fn new() -> Self {
        Self {
            stdout: Box::new(std::io::stdout()),
            stderr: Box::new(std::io::stderr()),
        }
    }

    pub fn write_stdout(&mut self, text: &str) -> ShellResult {
        self.stdout.write_all(text.as_bytes())?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn write_stderr(&mut self, text: &str) -> ShellResult {
        self.stderr.write_all(text.as_bytes())?;
        self.stderr.flush()?;

        Ok(())
    }

    pub fn redirect_stdout(&mut self, filepath: &str, append: bool) -> ShellResult {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(append)
            .truncate(!append)
            .open(filepath)?;
        self.stdout = Box::new(file);

        Ok(())
    }

    pub fn redirect_stderr(&mut self, filepath: &str, append: bool) -> ShellResult {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(append)
            .truncate(!append)
            .open(filepath)?;
        self.stderr = Box::new(file);

        Ok(())
    }
}
