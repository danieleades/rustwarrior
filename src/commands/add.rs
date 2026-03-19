use clap::Parser;
use rustwarrior_core::{Priority, Project, Store, Task};

use crate::store_ext::StoreExt;

/// Add a new task
#[derive(Debug, Parser)]
pub struct Add {
    /// The description of the task
    description: String,
    /// Priority level (1-4)
    #[clap(long, short)]
    priority: Option<Priority>,
    /// Project path, using dots to indicate subprojects
    #[clap(long)]
    project: Option<Project>,
}

impl Add {
    /// Run the add command
    pub fn run(self) -> anyhow::Result<()> {
        let mut store = Store::load_default()?;

        let mut task = Task::new(self.description);
        if let Some(p) = self.priority {
            task = task.with_priority(p);
        }
        if let Some(project) = self.project {
            task = task.with_project(project);
        }
        let id = store.push(task);
        store.save_default()?;
        println!("Added task {id}");
        Ok(())
    }
}
