use std::fmt::format;

use clap::{Parser, Subcommand};
use rusqlite::Connection;
use thiserror::Error;

use crate::crud::{create_task, read_task, update_task, delete_task, CrudError, Task};

pub enum LexerOk {
    Create(usize),
    Read(Vec<Task>),
    Update(Task),
    Delete(String),
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Invalid Command:\n\n {0}")]
    InputError(String),
    #[error(transparent)]
    CrudError(#[from] CrudError),
}

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "cargo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add { create_args: Vec<String> },
    Show { read_args: Option<String> },
    Finish { update_args: String },
    Delete { delete_args: String },
}

pub fn lexer(conn: &Connection) -> Result<LexerOk, LexerError> {
    let args = Cli::parse();

    match args.command {
        Commands::Add { create_args } => {
            let title = create_args.join(" ");
            if title.len() == 0 {
                return Err(LexerError::InputError("Please give a title".to_string()));
            } else {
                Ok(LexerOk::Create(create_task(conn, title)?))
            }
        }
        Commands::Show { read_args } => {
            let mut task_id = None;

            if let Some(read_args) = read_args {
                if let Ok(valid_number) = read_args.parse() {
                    task_id = Some(valid_number);
                }
            }
            Ok(LexerOk::Read(read_task(conn, task_id)?))
        }
        Commands::Finish { update_args } => {
            let task_id = update_args
                .parse()
                .map_err(|_| {
                    LexerError::InputError(format(format_args!("Not a number: {}", update_args)))
                })?;

            Ok(LexerOk::Update(update_task(&conn, task_id)?))
        }
        Commands::Delete { delete_args } => {
            let task_id = delete_args
                .parse()
                .map_err(|_| {
                    LexerError::InputError(format(format_args!("Not a number: {}", delete_args)))
                })?;

            Ok(LexerOk::Delete(delete_task(&conn, task_id)?))
        }
    }
}
