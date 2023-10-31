// database.rs

use rusqlite::Connection;
use std::env;
use std::fs;
use std::path::PathBuf;

// Setup database path _____________________________________________________________________________
fn database_dir() -> PathBuf {
    let database_dir = match env::consts::OS {
        "linux" | "macos" => match dirs::data_dir() {
            Some(xdg_data_home) => xdg_data_home.join("pocato/"),
            None => panic!("Can't access the ~/.local/share/ folder"),
        },
        "windows" => match dirs::data_local_dir() {
            Some(appdata) => appdata.join("pocato/"),
            None => panic!("Can't find the AppData directory."),
        },
        _ => panic!("Unsupported platform"),
    };

    if !database_dir.is_dir() {
        match fs::create_dir(&database_dir) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        };
    }
    database_dir
}

// Initialize database _____________________________________________________________________________
pub fn init_db() -> Connection {
    let database_file = database_dir().join("tasks.db");
    match Connection::open(database_file) {
        Ok(conn) => {
            create_table(&conn);
            conn
        }
        Err(err) => panic!("Could not initialize Database: {}", err),
    }
}

// Create table ___________________________________________________________________________________""
fn create_table(conn: &Connection) {
    let result = conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            uuid TEXT PRIMARY KEY, 
            id INTEGER, 
            title TEXT NOT NULL UNIQUE,
            state INTEGER
            )",
        (),
    );
    match result {
        Ok(_) => (),
        Err(err) => panic!("Could not create Table {}", err),
    }
}
