// error.rs

use rusqlite::Error;
use std::env::VarError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    // #[error("\x1b[31mInvalid Command: {0}\x1b[0m")]
    // InvalidCommand(#[from] clap::Error),
    #[error("\x1b[31mInvalid Command Arguments: {0}\x1b[0m")]
    InvalidCommandArguments(String),

    #[error("\x1b[31mInvalid Command Format: {0}\x1b[0m")]
    InvalidArgumentFormat(#[from] ParseIntError),

    #[error(transparent)]
    CrudError(#[from] CrudError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("\x1b[33mCan't locate config folder. Using default settings.\n
        Consider setting $POCATO_DIR:\n\x1b[0m{0}")]
    VarError(#[from] VarError),

    #[error("\x1b[31mTOML Serialization Error:\n\x1b[0m{0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("\x1b[31mTOML Deserialization Error:\n\x1b[0m{0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("\x1b[31m Read/Write Error:\n\x1b[0m{0}")]
    StdIOError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum CrudError {
    #[error("\x1b[31mRusqlite Error:\n\x1b[0m{0}")]
    DatabaseError(#[from] Error),
    #[error("\x1b[31mInput Error:\n\x1b[0m{0}")]
    TaskNotFound(String),
}
