//main.rs

// Version 0.2.0:
// -------------------------------------------------------------------------------------------------
// [ ] Refactor Task icons, states & lexer (not dry)
// [ ] Make the application more modular in preparation for the web app & gui
// [ ] Improve Error enums (consolidate them)
// [ ] More consistent error messages
// [ ] Add Tests for TDD/ CI (test driven development/ continuous intergration)
// [x] Separate out Tasks into its own module
// [x] Add "created, project & parent" fields to data base
// [x] Pick Nerd Font icons for task status representation
// [x] Add different task states
// [x] Display Tasks and messages nicer and more consistently
// [x] Confirmation of deletion
// [x] Id numbers get renumbered on delete
// -------------------------------------------------------------------------------------------------

// Version 0.3.0:
// -------------------------------------------------------------------------------------------------
// [ ] Check if Nerd font is available and set alternative icons if not
// [ ] Custom Lexer & Parser
// [ ] Implement Filter
// [ ] Implement Projects workflow
// [ ] Implement a planning workflow
// [ ] Add Tags Field
// [ ] EGUI
// -------------------------------------------------------------------------------------------------

mod crud;
mod lexer;
mod database;
mod error;
mod task;

fn main() {
    // Establish SQLite Database connection
    let conn = database::init_db();

    // Parse CLI arguments & call CRUD methods
    lexer::lexer_handler(&conn);
}
