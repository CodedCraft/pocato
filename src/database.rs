use rusqlite::Connection;

pub fn init_db() -> Connection {
    match Connection::open("tasks.db") {
        Ok(conn) => {
            create_table(&conn);
            conn
        }
        Err(err) => panic!("Could not initialize Database: {}", err),
    }
}

fn create_table(conn: &Connection) {
    let result = conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            uuid TEXT, 
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            title TEXT NOT NULL,
            status BOOLEAN DEFAULT FALSE
            )",
        (),
    );
    match result {
        Ok(_) => (),
        Err(err) => panic!("Could not create Table {}", err),
    }

    // Trigger be implemented in 0.2.0:
    // connection.execute(
    //     "CREATE TRIGGER IF NOT EXISTS increment_id
    //     AFTER INSERT
    //     ON tasks
    //     FOR EACH ROW
    //     BEGIN
    //         UPDATE tasks SET id = id + 1;
    //     END;",
    //     [],
    // )?;
}
