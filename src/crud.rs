// crud.rs

use dialoguer::Confirm;
use rusqlite::Connection;
use tabled::{
    settings::{object::Columns, Disable, Style},
    Table,};

use crate::error::CrudError;
use crate::task::*;

// CRUD methods (Create, Read, Update, Delete) -----------------------------------------------------
pub fn create_task(conn: &Connection, title: String) -> Result<String, CrudError> {
    let id: i64 = conn.query_row("SELECT COALESCE(MAX(id), 0) + 1 FROM tasks", [], |row| {
        row.get(0)
    })?;

    let task = Task::new(title.clone(), id);

    conn.execute(
        "INSERT INTO tasks
        (uuid, id, title, state, created, project, parent)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            task.uuid,
            task.id,
            task.title.clone(),
            task.state.to_string(),
            task.created,
            task.project,
            task.parent,
        ),
    )?;
    Ok(format!(
        "Added new task:\n  \x1b[1;34m{}\x1b[0m (#{})",
        task.title, task.id
    ))
}

pub fn read_task(conn: &Connection, task_id: Option<i64>) -> Result<String, CrudError> {
    let task_vec = get_tasks(conn, task_id)?;
    let task_table = build_task_table(task_vec);
    Ok(task_table)
}

pub fn update_task(
    conn: &Connection,
    task_id: i64,
    task_state: TaskState,
) -> Result<String, CrudError> {
    let task = &get_tasks(conn, Some(task_id))?[0];
    conn.execute(
        "UPDATE tasks SET state = ? WHERE id = ?",
        (task_state.to_string(), task_id),
    )?;
    Ok(format!(
        "{}:\n{}  \x1b[1;34m{}\x1b[0m (#{})",
        task_state.to_string(),
        task_state.get_icon(),
        task.title,
        task.id
    ))
}

pub fn delete_task(conn: &Connection, task_id: i64) -> Result<String, CrudError> {
    let task = &get_tasks(conn, Some(task_id))?[0];
    let confirmation_message = format!("Delete task {} '{}'? (yes/no)", task.id, task.title);
    match Confirm::new().with_prompt(confirmation_message).interact() {
        Ok(x) => {
            if x {
                // Delete task:
                conn.execute("DELETE FROM tasks WHERE id = ?", [task_id])?;
                // Renumber id numbers to close gaps created by a deletion:
                conn.execute(
            "UPDATE tasks SET id = (SELECT COUNT(*) FROM tasks t WHERE t.id < tasks.id) + 1",
            [],
    )?;
                return Ok(format!(
                    "Deleted:\n\x1b[34m{}\x1b[0m ({})",
                    task.title, task.id
                ));
            }
        }
        Err(err) => eprintln!("{}", err),
    }
    Ok("Task not deleted".to_string())
}

// Helper functions --------------------------------------------------------------------------------

fn build_task_table(tasks: Vec<Task>) -> String {
    let style = Style::rounded();
    let disable = Disable::column(
        Columns::new(3..), // disable range
                             // Columns::single(3), // disable single column
    );
    let table = Table::new(tasks).with(style).with(disable).to_string();
    table
}

fn get_tasks(conn: &Connection, task_id: Option<i64>) -> Result<Vec<Task>, CrudError> {
    let query = match task_id {
        Some(id) => format!("SELECT * FROM tasks WHERE id = {}", id),
        None => "SELECT * FROM tasks".to_string(),
    };

    let mut stmt = conn.prepare(&query)?;
    let tasks = stmt.query_map((), |row| {
        Ok(Task {
            uuid: row.get(0)?,
            id: row.get(1)?,
            title: row.get(2)?,
            state: TaskState::to_state(row.get(3)?),
            created: row.get(4)?,
            project: row.get(5)?,
            parent: row.get(6)?,
        })
    })?;

    let mut task_vec = Vec::new();

    for task in tasks {
        task_vec.push(task?);
    }

    if task_vec.is_empty() {
        return Err(CrudError::TaskNotFound("Task not found".to_string()));
    }

    Ok(task_vec)
}
