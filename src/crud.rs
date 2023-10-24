// crud.rs

use rusqlite::Connection;
use std::fmt;
use uuid::Uuid;

use crate::error::CrudError;

// Define Task _____________________________________________________________________________________
#[derive(Debug, Clone)]
pub struct Task {
    pub uuid: String,
    pub id: i64,
    pub title: String,
    pub status: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.status {
            "\x1b[32m\x1b[0m"
        } else {
            ""
        };

        // NerdFont Signs for Future reference:
        // --------------------------------------------------------------------
        // Finished: 󰱒 (nf-md-checkbox_outline)
        // Deleted:  󰛌 (nf-md-delete_empty)       󰅘 (nf-md-close_box_outline)
        // Started:  󰛲 (nf-md-minus_box_outline)  󱗝 (nf-md-circle_box_outline)
        // New:      󰿦 (nf-md-texture_box)        󰆢 (nf-md-crop_square)
        // Project:   (nf-oct-project_roadmap)
        // --------------------------------------------------------------------

        let title = format!("\x1b[1;34m{}\x1b[0m", self.title);
        write!(f, "{}  {} ({})", status, title, self.id)
    }
}

// CRUD methods (Create, Read, Update, Delete) -----------------------------------------------------
pub fn create_task(conn: &Connection, title: String) -> Result<String, CrudError> {
    let next_id: i64 = conn.query_row("SELECT COALESCE(MAX(id), 0) + 1 FROM tasks", [], |row| {
        row.get(0)
    })?;

    let task = Task {
        id: next_id,
        uuid: Uuid::new_v4().to_string(),
        title,
        status: false,
    };

    conn.execute(
        "INSERT INTO tasks (uuid, id, title, status) VALUES (?1, ?2, ?3, ?4)",
        (task.uuid, task.id, task.title.clone(), task.status),
    )?;
    Ok(format!("Added new task:\n\x1b[1;34m{}\x1b[0m", task.title))
}

pub fn read_task(conn: &Connection, task_id: Option<i64>) -> Result<String, CrudError> {
    let task_vec = get_tasks(conn, task_id)?;
    let task_list = build_task_list(task_vec);
    Ok(task_list)
}

pub fn update_task(conn: &Connection, task_id: i64) -> Result<String, CrudError> {
    let task = &get_tasks(conn, Some(task_id))?[0];
    conn.execute("UPDATE tasks SET status = true WHERE id = ?", [task_id])?;
    Ok(format!(
        "Finished:\n\x1b[1;34m{}\x1b[0m ({})",
        task.title, task.id
    ))
}

pub fn delete_task(conn: &Connection, task_id: i64) -> Result<String, CrudError> {
    let task = &get_tasks(conn, Some(task_id))?[0];
    conn.execute("DELETE FROM tasks WHERE id = ?", [task_id])?;
    // Renumber id numbers to close gaps created by a deletion
    conn.execute(
        "UPDATE tasks SET id = (SELECT COUNT(*) FROM tasks t WHERE t.id < tasks.id) + 1",
        [],
    )?;
    Ok(format!(
        "Deleted:\n\x1b[1;34m{}\x1b[0m ({})",
        task.title, task.id
    ))
}

// Helper functions ________________________________________________________________________________
fn build_task_list(tasks: Vec<Task>) -> String {
    let mut task_list = String::new();
    for task in tasks {
        task_list.push_str(&format!("{}\n", task));
    }
    task_list
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
            status: row.get(3)?,
        })
    })?;

    let mut task_vec = Vec::new();

    for task in tasks {
        task_vec.push(task?);
    }

    if task_vec.is_empty() {
        return Err(CrudError::TaskNotFound("No tasks found".to_string()));
    }

    Ok(task_vec)
}
