// task.rs

use chrono::prelude::*;
use core::fmt;
use tabled::Tabled;
use uuid::Uuid;
use colored::*;

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

// Define and implement Task states ----------------------------------------------------------------
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

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", TaskState::get_icon(&self))
    }
}

impl TaskState {
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
    pub fn get_icon(&self) -> ColoredString {
        // Alternative icons when Nerdfonts are not available:
        // ---------------------------------------------------
        // match self {
        //     TaskState::Pending => "[ ]".white(), 
        //     TaskState::Started => "[|]".yellow(),
        //     TaskState::Finished => "[√]".green(),  
        //     TaskState::Blocked => "[#]".blue(),   
        //     TaskState::Someday => "[~]".yellow(), 
        //     TaskState::Cancelled => "[x]".red(),  
        //     TaskState::Paused => "[-]".white(),   
        // }
        // ---------------------------------------------------

        match self {
            TaskState::Pending => "".white(), 
            TaskState::Started => "".yellow(),
            TaskState::Finished => "".green(),  
            TaskState::Blocked => "".blue(),   
            TaskState::Someday => "".yellow(), 
            TaskState::Cancelled => "".red(),  
            TaskState::Paused => "".white(),   
        }
    }
}
