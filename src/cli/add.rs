use clap::Parser;
use rustwarrior_core::{Priority, Task};

use crate::store::Store;

#[derive(Debug, Parser)]
pub struct Add {
    /// The description of the task
    description: String,
    #[clap(long, short)]
    priority: Option<Priority>,
}

impl Add {
    pub fn run(self) -> anyhow::Result<()> {
        let mut store = Store::load()?;

        let mut task = Task::new(self.description);
        if let Some(p) = self.priority {
            task = task.with_priority(p);
        }
        let id = store.push(task);
        store.save()?;
        println!("Added task {id}");
        Ok(())
    }
}
