use clap::{Parser, Subcommand};
use rusqlite::{Connection, Error, Result};
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Task {
    id: i64,
    title: String,
    status: bool,
}

// Clap CLI commands -------------------------------------------------------------------------------

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "cargo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Create { create_args: Vec<String> },
    // Create { create_args: String },
    Read,
    Update,
    Delete,
}
// -------------------------------------------------------------------------------------------------
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

// CRUD methods ------------------------------------------------------------------------------------
fn create_task(conn: &Connection, task: &Task) -> Result<usize, Error> {
    let result = conn.execute(
        "INSERT INTO tasks (id, title, status) VALUES (?1, ?2, ?3)",
        (task.id, task.title.clone(), task.status),
    )?;
    Ok(result)
}

fn read_task(conn: &Connection) -> Result<Vec<Task>, Error> {
    let mut stmt = conn.prepare("SELECT * FROM tasks")?;

    let tasks = stmt.query_map((), |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            status: row.get(2)?,
        })
    })?;

    let mut task_array = Vec::new();

    for task in tasks {
        match task {
            Ok(task) => task_array.push(task),
            Err(err) => println!("Error: {:?}", err),
        }
    }

    Ok(task_array)
}

fn update_task(conn: &Connection) -> Result<usize, Error> {
    let result = conn.execute("UPDATE tasks SET status = true", ())?;
    Ok(result)
}

fn delete_task(conn: &Connection) -> Result<usize, Error> {
    // let result = conn.execute("DELETE FROM tasks WHERE (id) = 1", ())?;
    let result = conn.execute("DROP TABLE tasks", ())?;
    Ok(result)
}
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

    // Handle Cli args -----------------------------------------------------------------------------
    let args = Cli::parse();
    match args.command {
        Commands::Create { create_args } => {
            let task = Task {
                id: Uuid::new_v4 as i64,
                title: create_args.join(" "),
                status: false,
            };

            let create_result = create_task(&conn, &task);
            match create_result {
                Ok(_) => println!("Added: {}", &task.title),
                Err(err) => println!("Could not add task. {}", err),
            }
        }
        Commands::Read => {
            let read_result = read_task(&conn);
            match read_result {
                Ok(tasks) => {
                    for item in tasks {
                        println!("{:?}", item);
                    }
                }
                Err(err) => println!("Could not retrieve task: {}", err),
            }
        }
        Commands::Update => {
            let update_result = update_task(&conn);
            match update_result {
                Ok(_) => println!("Task succesfully updated"),
                Err(err) => println!("Could not update task: {}", err),
            }
        }
        Commands::Delete => {
            let delete_result = delete_task(&conn);
            match delete_result {
                Ok(_) => println!("Successfully deleted"),
                Err(err) => println!("Could not delete task: {}", err),
            }
        } // ---------------------------------------------------------------------------------------------
    }
}
