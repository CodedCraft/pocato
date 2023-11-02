// lexer.rs

use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::error::LexerError;
use crate::crud::*;
use crate::task::*;

// Clap Setup ______________________________________________________________________________________
#[derive(Debug, Parser)]
#[command(name = "cargo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add { create_args: Vec<String> },
    Show { read_args: Option<String> },
    Start { task_id: String },
    Block { task_id: String },
    Someday { task_id: String },
    Cancel { task_id: String },
    Pause { task_id: String },
    Finish { update_args: String },
    Delete { delete_args: String },
}

// Public Lexer interface __________________________________________________________________________
pub fn lexer_handler(conn: &Connection) {
    match parse_cli(conn) {
        Ok(success) => println!("{}", success),
        Err(err) => eprintln!("{}", err),
    };
}

// Private parser functions ________________________________________________________________________
fn parse_cli(conn: &Connection) -> Result<String, LexerError> {
    let args = Cli::parse();

    match args.command {
        Commands::Add { create_args } => {
            let title = create_args.join(" ");
            if title.is_empty() {
                return Err(LexerError::InputError("Task name missing, please enter a name.".to_string()));
            } else {
                Ok(create_task(conn, title)?)
            }
        }

        Commands::Show { read_args } => {
            let task_id = read_args.and_then(|string_id| parse_number(string_id));
            Ok(read_task(conn, task_id)?)
        }

        Commands::Start { task_id } => {
            if let Some(task_id) = parse_number(task_id) {
                return Ok(update_task(&conn, task_id, TaskState::Started)?);
            } else {
                return Err(LexerError::InputError(
                    "No Task ID, please enter a number".to_string(),
                ));
            }
        }

        Commands::Block { task_id } => {
            if let Some(task_id) = parse_number(task_id) {
                return Ok(update_task(&conn, task_id, TaskState::Blocked)?);
            } else {
                return Err(LexerError::InputError(
                    "No Task ID, please enter a number".to_string(),
                ));
            }
        }

        Commands::Someday { task_id } => {
            if let Some(task_id) = parse_number(task_id) {
                return Ok(update_task(&conn, task_id, TaskState::Someday)?);
            } else {
                return Err(LexerError::InputError(
                    "No Task ID, please enter a number".to_string(),
                ));
            }
        }

        Commands::Cancel { task_id } => {
            if let Some(task_id) = parse_number(task_id) {
                return Ok(update_task(&conn, task_id, TaskState::Cancelled)?);
            } else {
                return Err(LexerError::InputError(
                    "No Task ID, please enter a number".to_string(),
                ));
            }
        }

        Commands::Pause { task_id } => {
            if let Some(task_id) = parse_number(task_id) {
                return Ok(update_task(&conn, task_id, TaskState::Paused)?);
            } else {
                return Err(LexerError::InputError(
                    "No Task ID, please enter a number".to_string(),
                ));
            }
        }

        Commands::Finish { update_args } => {
            if let Some(task_id) = parse_number(update_args) {
                return Ok(update_task(&conn, task_id, TaskState::Finished)?);
            } else {
                return Err(LexerError::InputError(
                    "No Task ID, please enter a number".to_string(),
                ));
            }
        }

        Commands::Delete { delete_args } => {
            if let Some(task_id) = parse_number(delete_args) {
                return Ok(delete_task(&conn, task_id)?);
            }
            else {

            
            return Err(LexerError::InputError(
                "No Task ID, please enter a number".to_string(),
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
