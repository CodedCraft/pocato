// error.rs

use thiserror::Error;
use rusqlite::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("\x1b[31mInvalid Command:\n\x1b[0m{0}")]
    InputError(String),
    #[error(transparent)]
    CrudError(#[from] CrudError),
}

#[derive(Debug, Error)]
pub enum CrudError {
    #[error("Rusqlite Error:\n {0}")]
    RusqliteError(#[from] Error),
    #[error("Input Error:\n {0}")]
    TaskNotFound(String),
}
