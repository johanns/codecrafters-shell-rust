mod commands;
mod error;
mod repl;
mod shell;
mod output;

use error::ShellResult;
use repl::Repl;

fn main() -> ShellResult {
    let mut repl = Repl::new();
    repl.run()
}
