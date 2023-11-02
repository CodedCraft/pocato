// error.rs

use std::num::ParseIntError;
use thiserror::Error;
use rusqlite::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("\x1b[31mInvalid Command:\n\x1b[0m{0}")]
    InputError(String),
    #[error(transparent)]
    CrudError(#[from] CrudError),
    #[error("\x1b[31mInvalid Command:\n\x1b[0mNot a valid number")]
    ParseIntError(#[from] ParseIntError)

}

#[derive(Debug, Error)]
pub enum CrudError {
    #[error("\x1b[31mRusqlite Error:\n\x1b[0m{0}")]
    RusqliteError(#[from] Error),
    #[error("\x1b[31mInput Error:\n\x1b[0m{0}")]
    TaskNotFound(String),
}
