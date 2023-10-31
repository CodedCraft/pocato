//main.rs

// Version 0.2.0:
// -------------------------------------------------------------------------------------------------
// [ ] Pick Nerd Font icons for task status representation
// [ ] Add fields to data base
// [ ] Organise modules
// [ ] Improve Error enums (consolidate them)
// [ ] More consistent error messages
// [ ] Make the application more modular in preparation for the web app & gui
// [ ] Add Tests for TDD/ CI (test driven development/ continuous intergration)
// [x] Add different task states
// [x] Display Tasks and messages nicer and more consistently
// [x] Confirmation of deletion
// [x] Id numbers get renumbered on delete
// -------------------------------------------------------------------------------------------------

// Version 0.3.0:
// -------------------------------------------------------------------------------------------------
// [ ] EGUI
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
