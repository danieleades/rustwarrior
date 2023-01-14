use clap::Parser;
use rustwarrior::{Store, Task};

#[derive(Debug, Parser)]
pub struct Add {
    /// The description of the task
    description: String,
}

impl Add {
    pub fn run(self) -> anyhow::Result<()> {
        let mut store = Store::load()?;

        let task = Task::new(self.description);
        let id = store.push(task);
        store.save()?;
        println!("Added task {id}");
        Ok(())
    }
}
