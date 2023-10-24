//main.rs

// Version 0.2.0:
// -------------------------------------------------------------------------------------------------
// [ ] Confirmation of deletion
// [ ] Improve Error enums (consolidate them)
// [ ] More consistent error messages
// [ ] Display Task nicer and more consistently
// [ ] Make the application more modular in preparation for the web app & gui
// [ ] Add Tests for TDD/ CI (test driven development/ continuous intergration)
// [ ] Add different task states
// [ ] Pick Nerd Font icons for task status representation
// [x] Id numbers get renumbered on delete
// -------------------------------------------------------------------------------------------------

// Version 0.3.0:
// -------------------------------------------------------------------------------------------------
// [ ] Axum Web App
// -------------------------------------------------------------------------------------------------


mod crud;
mod lexer;
mod database;
mod error;

fn main() {
    // Establish SQLite Database connection
    let conn = database::init_db();

    // Parse CLI arguments & call CRUD methods
    lexer::lexer_handler(&conn);
}
