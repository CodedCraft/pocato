// MVP version 0.1.0:
// -------------------------------------------------------------------------------------------------
// [ ] Code is not dry
// [ ] Add comments
// [ ] Create Readme
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
// [ ] Add Tests for TDD/ CI (test driven development/ continuous intergration)
// -------------------------------------------------------------------------------------------------

mod crud;
mod lexer;
mod database;

fn main() {
    // Establish SQLite Database connection
    let conn = database::init_db();

    // Parse CLI arguments
    lexer::lexer_handler(&conn);
}
