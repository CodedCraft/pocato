use rusqlite::{Connection, Error};
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CrudError {
    #[error("Rusqlite Error:\n {0}")]
    RusqliteError(#[from] Error),
    #[error("Input Error:\n {0}")]
    TaskNotFound(String),
}

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
        write!(f, "({})  {} {}",self.id, status, title)
    }
}

pub fn create_task(conn: &Connection, title: String) -> Result<usize, CrudError> {
    let task = Task {
        id: 0,
        uuid: Uuid::new_v4().to_string(),
        title,
        status: false,
    };
    let result = conn.execute(
        "INSERT INTO tasks (uuid, title, status) VALUES (?1, ?2, ?3)",
        (task.uuid, task.title.clone(), task.status),
    )?;
    Ok(result)
}

pub fn read_task(conn: &Connection, id: Option<i64>) -> Result<Vec<Task>, CrudError> {
    // let mut query = "SELECT * FROM tasks WHERE status=false".to_string();
    let mut query = "SELECT * FROM tasks".to_string();

    if let Some(id) = id {
        query.push_str(&format!(" AND id={}", id))
    }

    let mut stmt = conn.prepare(&query)?;
    let tasks = stmt.query_map((), |row| {
        Ok(Task {
            uuid: row.get(0)?,
            id: row.get(1)?,
            title: row.get(2)?,
            status: row.get(3)?,
        })
    })?;

    let mut task_array = Vec::new();

    for task in tasks {
        task_array.push(task?);
    }

    match task_array.len() {
        0 => return Err(CrudError::TaskNotFound("No such Task".to_string())),
        _ => return Ok(task_array),
    }
}

pub fn update_task(conn: &Connection, task_id: i64) -> Result<Task, CrudError> {
    let query = format!("SELECT * FROM tasks WHERE id = {}", task_id);
    let mut stmt = conn.prepare(&query)?;
    let mut task_to_delete = stmt.query_map((), |row| {
        Ok(Task {
            uuid: row.get(0)?,
            id: row.get(1)?,
            title: row.get(2)?,
            status: row.get(3)?,
        })
    })?;

    if let Some(task) = task_to_delete.next() {
        conn.execute("UPDATE tasks SET status = true WHERE id = ?", [task_id])?;
        return Ok(task?);
    } else {
        return Err(CrudError::TaskNotFound("No such Task".to_string()));
    }
}

pub fn delete_task(conn: &Connection, task_id: i64) -> Result<String, CrudError> {
    // Check if the task exists
    let task_exists = conn.query_row("SELECT 1 FROM tasks WHERE id = ?", [task_id], |_row| Ok(()));

    match task_exists {
        Ok(_) => {
            // Task exists, proceed to delete
            conn.execute("DELETE FROM tasks WHERE id = ?", [task_id])?;
            Ok(format!("Task deleted {}", task_id))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // Task doesn't exist
            Err(CrudError::TaskNotFound("No such Task".to_string()))
        }
        Err(err) => {
            // Handle other errors
            Err(CrudError::RusqliteError(err))
        }
    }
}
