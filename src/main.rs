//main.rs

// MVP version 0.1.0:
// -------------------------------------------------------------------------------------------------
// [ ] Test
// [x] Move database location to XDG Base Directory Specification compliant location
// [x] Create Readme
// [x] Add comments
// [x] Dry up Code
// [x] Make a function that checks if an task id is present => get_task() / get_all_tasks()
// [x] Id numbers are unwieldy (uuid)
// [x] Displaying tasks in a nice way
// [x] Change the read_task method so it only shows tasks that are not finished
// [x] Finishing a task doesn't confirm the task name
// [x] Code is (especially the CLI command handling) not yet separated out
// -------------------------------------------------------------------------------------------------

// Version 0.2.0:
// -------------------------------------------------------------------------------------------------
// [ ] Id numbers get renumbered on delete (Implement SQL Trigger)
// [ ] Confirmation of deletion
// [ ] Improve Error enums (consolidate them)
// [ ] Display Task nicer and more consistently
// [ ] Make the application more modular in preparation for the web app & gui
// [ ] Add Tests for TDD/ CI (test driven development/ continuous intergration)
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
