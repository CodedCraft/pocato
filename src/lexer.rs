// lexer.rs

use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::crud::*;
use crate::error::LexerError;
use crate::task::TaskState;

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
    Show { task_id: Option<String> },
    Start { task_id: String },
    Block { task_id: String },
    Someday { task_id: String },
    Cancel { task_id: String },
    Pause { task_id: String },
    Finish { task_id: String },
    Delete { task_id: String },
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
                return Err(LexerError::InputError(
                    "Task name missing, please enter a name.".to_string(),
                ));
            } else {
                Ok(create_task(conn, title)?)
            }
        }

        Commands::Show { task_id } => {
            let task_id = match task_id {
                Some(task_id) => Some(parse_num(task_id)?),
                None => None,
            };
            Ok(read_task(conn, task_id)?)
        }

        Commands::Start { task_id } => {
            Ok(update_task(conn, parse_num(task_id)?, TaskState::Started)?)
        }

        Commands::Block { task_id } => {
            Ok(update_task(conn, parse_num(task_id)?, TaskState::Blocked)?)
        }

        Commands::Someday { task_id } => {
            Ok(update_task(conn, parse_num(task_id)?, TaskState::Someday)?)
        }

        Commands::Cancel { task_id } => Ok(update_task(
            conn,
            parse_num(task_id)?,
            TaskState::Cancelled,
        )?),

        Commands::Pause { task_id } => {
            Ok(update_task(conn, parse_num(task_id)?, TaskState::Paused)?)
        }

        Commands::Finish { task_id } => {
            Ok(update_task(conn, parse_num(task_id)?, TaskState::Finished)?)
        }

        Commands::Delete { task_id } => Ok(delete_task(conn, parse_num(task_id)?)?),
    }
}

fn parse_num(num_string: String) -> Result<i64, LexerError> {
    Ok(num_string.parse::<i64>()?)
}
