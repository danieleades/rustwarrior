use clap::Parser;
use rustwarrior::{Store, Task};

#[derive(Debug, Parser)]
pub struct Add {
    /// The description of the task
    description: String,
}

impl Add {
    pub fn run(self) -> anyhow::Result<()> {
        let mut store = Store::load_from_dir_default()?;
        let id = store.first_missing_id();

        let task = Task::new(id, self.description);
        store.push(task);
        store.save_to_dir_default()?;
        println!("Added task {id}");
        Ok(())
    }
}
