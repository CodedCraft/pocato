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
        write!(f, "({})  {} {}", self.id, status, title)
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

pub fn read_task(conn: &Connection, task_id: Option<i64>) -> Result<Vec<Task>, CrudError> {
    if let Some(task_id) = task_id {
        let task = get_task(conn, task_id)?;
        let mut task_vec = Vec::new();
        task_vec.push(task);
        return Ok(task_vec);
    } else {
        let task_vec = get_all_tasks(conn)?;
        return Ok(task_vec);
    }
}

pub fn update_task(conn: &Connection, task_id: i64) -> Result<String, CrudError> {
    match get_task(conn, task_id) {
        Ok(task) => {
            conn.execute("UPDATE tasks SET status = true WHERE id = ?", [task_id])?;
            Ok(format!("\x1b[1;34m{}\x1b[0m (Id: {})", task.title, task.id))
        }
        Err(err) => Err(err),
    }
}

pub fn delete_task(conn: &Connection, task_id: i64) -> Result<String, CrudError> {
    match get_task(conn, task_id) {
        Ok(task) => {
            conn.execute("DELETE FROM tasks WHERE id = ?", [task_id])?;
            Ok(format!("\x1b[1;34m{}\x1b[0m (ID: {})", task.title, task.id))
        }
        Err(err) => Err(err),
    }
}

fn get_task(conn: &Connection, task_id: i64) -> Result<Task, CrudError> {
    let query = format!("SELECT * FROM tasks WHERE id = {}", task_id);
    let mut stmt = conn.prepare(&query)?;
    let task = stmt.query_row((), |row| {
        Ok(Task {
            uuid: row.get(0)?,
            id: row.get(1)?,
            title: row.get(2)?,
            status: row.get(3)?,
        })
    });

    match task {
        Ok(task) => Ok(task),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err(CrudError::TaskNotFound("No such Task".to_string()))
        }
        Err(err) => Err(CrudError::RusqliteError(err)),
    }
}

fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>, CrudError> {
    let query = "SELECT * FROM tasks".to_string();
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
    Ok(task_vec)
}
