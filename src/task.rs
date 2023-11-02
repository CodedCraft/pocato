use chrono::prelude::*;
use core::fmt;
use tabled::Tabled;
use uuid::Uuid;

// Define and implement Task -----------------------------------------------------------------------
#[derive(Debug, Clone, Tabled)]
pub struct Task {
    #[tabled(rename = "📝")]
    pub state: TaskState,
    #[tabled(rename = "\x1b[1;34mTask\x1b[0m")]
    pub title: String,
    #[tabled(rename = "\x1b[1;34mID\x1b[0m")]
    pub id: i64,
    pub uuid: String,
    pub created: String,
    pub project: bool,
    pub parent: String,
    // tags: Vec<String>,
}

impl Task {
    pub fn new(title: String, id: i64) -> Self {
        Self {
            state: TaskState::Pending,
            title,
            id,
            uuid: Uuid::new_v4().to_string(),
            created: Utc::now().to_string(),
            project: false,
            parent: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Pending,
    Started,
    Finished,
    Blocked,
    Someday,
    Cancelled,
    Paused,
}

// NerdFont Signs for Future reference:
// --------------------------------------------------------------------
// Finished: 󰱒 (nf-md-checkbox_outline)    (nf-oct-checkbox)
// Deleted:  󰛌 (nf-md-delete_empty)       󰅘 (nf-md-close_box_outline)
// Started:  󰛲 (nf-md-minus_box_outline)  󱗝 (nf-md-circle_box_outline)
// New:      󰿦 (nf-md-texture_box)        󰆢 (nf-md-crop_square)
// Project:   (nf-oct-project_roadmap)
// --------------------------------------------------------------------

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let icon = match self {
        //     TaskState::Pending  => "\x1b[39m[ ]\x1b[0m",
        //     TaskState::Started  => "\x1b[33m[|]\x1b[0m",
        //     TaskState::Finished => "\x1b[32m[√]\x1b[0m",
        //     TaskState::Blocked => "\x1b[32m[#]\x1b[0m", //nf-fa-check_square_o
        //     TaskState::Someday => "\x1b[32m[~]\x1b[0m", //nf-fa-check_square_o
        //     TaskState::Cancelled => "\x1b[32m[x]\x1b[0m", //nf-fa-check_square_o
        //     TaskState::Pause => "\x1b[32m[-]\x1b[0m", //nf-fa-check_square_o
        // };
        let icon = match self {
            TaskState::Pending => "\x1b[39m\x1b[0m",  //nf-fa-square_o
            TaskState::Started => "\x1b[33m\x1b[0m",  //nf-fa-pencil_square_o
            TaskState::Finished => "\x1b[32m\x1b[0m", //nf-fa-check_square_o
            TaskState::Blocked => "\x1b[34m\x1b[0m",
            TaskState::Someday => "\x1b[32m\x1b[0m",
            TaskState::Cancelled => "\x1b[31m\x1b[0m",
            TaskState::Paused => "\x1b[32m\x1b[0m",
        };
        write!(f, "{}", icon)
    }
}

impl TaskState {
    pub fn to_string(&self) -> String {
        match self {
            TaskState::Pending => "Pending".to_string(),
            TaskState::Started => "Started".to_string(),
            TaskState::Finished => "Finished".to_string(),
            TaskState::Blocked => "Blocked".to_string(),
            TaskState::Someday => "Someday".to_string(),
            TaskState::Cancelled => "Cancelled".to_string(),
            TaskState::Paused => "Paused".to_string(),
        }
    }

    pub fn to_state(text: String) -> TaskState {
        match text.as_str() {
            "Pending" => return TaskState::Pending,
            "Started" => TaskState::Started,
            "Finished" => TaskState::Finished,
            "Blocked" => TaskState::Blocked,
            "Someday" => TaskState::Someday,
            "Cancelled" => TaskState::Cancelled,
            "Paused" => TaskState::Paused,
            _ => unreachable!("Task state does not exist"),
        }
    }
    pub fn get_icon(&self) -> String {
        match self {
            TaskState::Pending => "\x1b[39m\x1b[0m".to_string(),
            TaskState::Started => "\x1b[33m\x1b[0m".to_string(),
            TaskState::Finished => "\x1b[32m\x1b[0m".to_string(),
            TaskState::Blocked => "\x1b[34m\x1b[0m".to_string(),
            TaskState::Someday => "\x1b[32m\x1b[0m".to_string(),
            TaskState::Cancelled => "\x1b[31m\x1b[0m".to_string(),
            TaskState::Paused => "\x1b[32m\x1b[0m".to_string(),
        }
    }
}
