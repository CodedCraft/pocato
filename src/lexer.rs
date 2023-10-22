// lexer.rs

use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::error::LexerError;
use crate::crud::*;

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
    Finish { update_args: String },
    Delete { delete_args: String },
}

// Public Crud interface ____________________________________________________________________________
pub fn lexer_handler(conn: &Connection) {
    match parse_cli(conn) {
        Ok(success) => println!("{}", success),
        Err(err) => eprintln!("{}", err),
    };
}

// Private Crud functions ___________________________________________________________________________
fn parse_cli(conn: &Connection) -> Result<String, LexerError> {
    let args = Cli::parse();

    match args.command {
        Commands::Add { create_args } => {
            let title = create_args.join(" ");
            if title.is_empty() {
                return Err(LexerError::InputError("Task name missing, please enter a name".to_string()));
            } else {
                Ok(create_task(conn, title)?)
            }
        }

        Commands::Show { read_args } => {
            let task_id = read_args.and_then(|string_id| parse_number(string_id));
            Ok(read_task(conn, task_id)?)
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
