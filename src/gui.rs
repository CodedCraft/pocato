//gui.rs

mod crud;
mod database;
mod error;
mod lexer;
mod task;

fn main() {
    // Establish SQLite Database connection
    let _conn = database::init_db();
}
