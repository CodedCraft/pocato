//gui.rs

mod crud;
mod database;
mod error;
mod lexer;
mod config;
mod task;


fn main() {
    // Establish SQLite Database connection
    let conn = database::init_db();

    // Parse CLI arguments & call CRUD methods
    lexer::lexer_handler(&conn);
}
