use std::time::Duration;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSch{
    
    pub task_name: String,
    pub task_hours: u64,
    pub exec_path: String,

}
#[derive(Debug, Serialize)]
pub struct TaskInfo{
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub last_run_time: Duration,
    pub next_run_time: Duration
} 