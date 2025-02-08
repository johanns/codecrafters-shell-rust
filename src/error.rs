use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("{0}")]
    Command(String),

    #[error("Command failed with exit code {code}: {message}")]
    CommandFailed { code: i32, message: String },

    #[error("Exit requested with code: {0}")]
    Exit(i32),
}

pub type ShellResult<T = ()> = Result<T, ShellError>;
