mod commands;
mod error;
mod repl;
mod shell;

use error::ShellResult;
use repl::Repl;

fn main() -> ShellResult {
    let mut repl = Repl::new();
    repl.run()
}
