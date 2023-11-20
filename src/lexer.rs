// lexer.rs

use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::crud::*;
use crate::error::CliError;
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
    let args = Cli::parse();
    match parse_cli(conn, args) {
        Ok(success) => println!("{}", success),
        Err(err) => eprintln!("{}", err),
    };
}

// Private parser functions ________________________________________________________________________
fn parse_cli(conn: &Connection, args: Cli) -> Result<String, CliError> {
    match args.command {
        Commands::Add { create_args } => {
            let title = create_args.join(" ");
            if title.is_empty() {
                return Err(CliError::InvalidCommandArguments(
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

fn parse_num(num_string: String) -> Result<i64, CliError> {
    Ok(num_string.parse::<i64>()?)
}
#[cfg(test)]
#[test]
fn test_commands() {
    let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
    crate::database::create_table(&conn);

    let mut args_to_test = Vec::new();

    args_to_test.push((
        vec!["pct", "add", "Clean room"],
        Ok("Added new task:\n\u{f096}  \u{1b}[1;34mClean room\u{1b}[0m (#1)"),
    ));

    args_to_test.push((
        vec!["pct", "add", ""],
        Err(CliError::InvalidCommandArguments(
            "Task name missing, please enter a name.".to_string(),
        )),
    ));

    args_to_test.push((
        vec!["pct", "show", "1"],
        Ok("╭────┬────────────┬────╮\n│ 📝 │ \u{1b}[1;34mTask\u{1b}[0m       │ \u{1b}[1;34mID\u{1b}[0m │\n├────┼────────────┼────┤\n│ \u{1b}[37m\u{f096}\u{1b}[0m  │ Clean room │ 1  │\n╰────┴────────────┴────╯")
    ));

    args_to_test.push((
        vec!["pct", "show"],
        Ok("╭────┬────────────┬────╮\n│ 📝 │ \u{1b}[1;34mTask\u{1b}[0m       │ \u{1b}[1;34mID\u{1b}[0m │\n├────┼────────────┼────┤\n│ \u{1b}[37m\u{f096}\u{1b}[0m  │ Clean room │ 1  │\n╰────┴────────────┴────╯")
    ));

    args_to_test.push((
        vec!["pct", "show", "2"],
        Err(CliError::CrudError(crate::error::CrudError::TaskNotFound(
            "Task not found".to_string(),
        ))),
    ));

    if let Err(parse_error) = "df".parse::<i64>() {
        args_to_test.push((
            vec!["pct", "show", "not_valid_number"],
            Err(CliError::InvalidArgumentFormat(parse_error)),
        ));
    }

    args_to_test.push((
        vec!["pct", "start", "1"],
        Ok("Started:\n\u{1b}[33m\u{f044}\u{1b}[0m  \u{1b}[1;34mClean room\u{1b}[0m (#1)"),
    ));

    args_to_test.push((
        vec!["pct", "block", "1"],
        Ok("Blocked:\n\u{1b}[34m\u{f256}\u{1b}[0m  \u{1b}[1;34mClean room\u{1b}[0m (#1)"),
    ));

    args_to_test.push((
        vec!["pct", "someday", "1"],
        Ok("Someday:\n\u{1b}[33m\u{f006}\u{1b}[0m  \u{1b}[1;34mClean room\u{1b}[0m (#1)"),
    ));

    args_to_test.push((
        vec!["pct", "cancel", "1"],
        Ok("Cancelled:\n\u{1b}[31m\u{f014}\u{1b}[0m  \u{1b}[1;34mClean room\u{1b}[0m (#1)"),
    ));

    args_to_test.push((
        vec!["pct", "pause", "1"],
        Ok("Paused:\n\u{1b}[37m\u{f520}\u{1b}[0m  \u{1b}[1;34mClean room\u{1b}[0m (#1)"),
    ));

    args_to_test.push((
        vec!["pct", "finish", "1"],
        Ok("Finished:\n\u{1b}[32m\u{f046}\u{1b}[0m  \u{1b}[1;34mClean room\u{1b}[0m (#1)"),
    ));

    args_to_test.push((
        vec!["pct", "delete", "1"],
        Ok("Deleted:\n\u{1b}[34mClean room\u{1b}[0m (#1)"),
    ));

    // Define a custom macro for assertions
    macro_rules! assert_result {
        ($actual:expr, $expected:expr) => {
            match ($actual, $expected) {
                (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
                (Err(actual), Err(expected)) => {
                    assert_eq!(actual.to_string(), expected.to_string())
                }
                _ => unreachable!(),
            }
        };
    }

    for (arg, expected_result) in args_to_test {
        assert_result!(parse_cli(&conn, Cli::parse_from(arg)), expected_result);
    }
}
