use clap::{Parser, Subcommand};
use rusqlite::Connection;
use thiserror::Error;

use crate::crud::*;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("\x1b[31mInvalid Command:\n\x1b[0m{0}")]
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

pub fn lexer_handler(conn: &Connection) {
    match parse_cli(conn) {
        Ok(success) => println!("{}", success),
        Err(err) => eprintln!("{}", err),
    };
}

fn parse_cli(conn: &Connection) -> Result<String, LexerError> {
    let args = Cli::parse();

    match args.command {
        Commands::Add { create_args } => {
            let title = create_args.join(" ");
            if title.len() == 0 {
                return Err(LexerError::InputError(
                    "Task name missing, please enter a name".to_string(),
                ));
            } else {
                Ok(create_task(conn, title)?)
            }
        }

        Commands::Show { read_args } => {
            let mut task_id = None;
            if let Some(read_args) = read_args {
                if let Ok(valid_number) = read_args.parse() {
                    task_id = Some(valid_number);
                }
            }
            read_task(conn, task_id)?;
            return Ok("".to_string());
        }

        Commands::Finish { update_args } => {
            if let Some(task_id) = parse_number(update_args) {
                return Ok(update_task(&conn, task_id)?);
            } else {
                return Err(LexerError::InputError(
                    "No Task Id, please enter a number".to_string(),
                ));
            }
        }

        Commands::Delete { delete_args } => {
            if let Some(task_id) = parse_number(delete_args) {
                return Ok(delete_task(&conn, task_id)?);
            }
            else {

            
            return Err(LexerError::InputError(
                "No Task Id, please enter a number".to_string(),
            ));
            }
        }
    }
}

fn parse_number(num_string: String) -> Option<i64> {
    match num_string.parse::<i64>() {
        Ok(parsed_num) => Some(parsed_num),
        Err(err) => {
            eprintln!("Parsing failed: {}", err);
            None
        }
    }
}
