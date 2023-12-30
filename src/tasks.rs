use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, Result, Seek, SeekFrom};
use std::path::PathBuf;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    // Write the modified task list back into the file
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;
    // remove the task
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid task id",
        ));
    }
    tasks.remove(task_position - 1);

    // rewind and reset file
    file.set_len(0)?;

    // write updated tasks in file
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // open and print tasks
    // open the file
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // get the tasks
    let tasks = collect_tasks(&file)?;

    // enumerate and display tasks
    if tasks.is_empty() {
        println!("No tasks to display");
    } else {
        let mut order = 1u32;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // get the tasks from file
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    // rewind the file
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}
