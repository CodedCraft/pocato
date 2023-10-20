// #![allow(unused)]
use rusqlite::{Connection, Error, Result};

mod crud;
mod lexer;
use lexer::lexer;

// Todo:
// -------------------------------------------------------------------------------------------------
// [x] Displaying tasks in a nice way                                                              |
// [ ] Id numbers are unwieldy (uuid)                                                              |
// [ ] Make a function that checks if an task id is present                                        |
// [ ] Code is not dry                                                                             |
// [ ] Add comments to functions or maybenot                                                       |
// [x] Change the read_task method so it only shows tasks that are not finished                    |
// [x] Finishing a task doesn't confirm the task name                                              |
// [x] Code is (especially the CLI command handling) not yet separated out                         |
// -------------------------------------------------------------------------------------------------

fn main() {
    // Initialize SQLite Database ------------------------------------------------------------------
    let database_init = init_db();
    let conn: Connection;

    match database_init {
        Ok(connection) => {
            conn = connection;
        }
        Err(err) => panic!("Could not initialize Database: {}", err),
    }
    // ---------------------------------------------------------------------------------------------

    let command_result = lexer(&conn);

    match command_result {
        Ok(lexer::LexerOk::Create(rows)) => println!("Successfully added {} rows", rows),
        Ok(lexer::LexerOk::Read(tasks)) => {
            for task in tasks {
                println!("{}", task);

            }
        }
        Ok(lexer::LexerOk::Update(result)) => println!("Task finished: {}", result),
        Ok(lexer::LexerOk::Delete(result)) => println!("{}", result),
        Err(err) => println!("{}", err),
    }
}

fn init_db() -> Result<Connection, Error> {
    let connection = Connection::open("tasks.db")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INT PRIMARY KEY,
            title TEXT NOT NULL,
            status BOOLEAN DEFAULT FALSE
            )",
        (),
    )?;
    Ok(connection)
}
