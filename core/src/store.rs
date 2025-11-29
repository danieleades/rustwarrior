use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    ops::Deref,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::task::Task;

/// Path resolution for task storage
pub mod paths;

/// A collection of [`Tasks`](Task).
///
/// These are represented as [`OpenTasks`](OpenTask), which are simply a wrapper
/// around a [`Task`] that adds a short ID field.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Store {
    open_tasks: Vec<OpenTask>,
}

impl Store {
    /// Create a new empty [`Store`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Load tasks from a specific file path
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let open_tasks = load_tasks_from_file(path)?;
        Ok(Self { open_tasks })
    }

    /// Save tasks to a specific file path
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save_to_path(&self, path: impl AsRef<Path>) -> io::Result<()> {
        save_tasks_to_file(&self.open_tasks, path)
    }

    /// Add a [`Task`] to the [`Store`].
    ///
    /// Returns the new ID associated with the open [`Task`].
    pub fn push(&mut self, task: Task) -> usize {
        let id = self.first_missing_id();
        let open_task = OpenTask { id, task };
        self.open_tasks.push(open_task);
        id
    }

    /// Delete a task by ID
    ///
    /// Returns the deleted task if found, otherwise `None`.
    pub fn delete(&mut self, id: usize) -> Option<OpenTask> {
        self.open_tasks
            .iter()
            .position(|t| t.id == id)
            .map(|idx| self.open_tasks.remove(idx))
    }

    /// Get a task by ID
    ///
    /// Returns a reference to the task if found, otherwise `None`.
    #[must_use]
    pub fn get(&self, id: usize) -> Option<&OpenTask> {
        self.open_tasks.iter().find(|t| t.id == id)
    }

    /// Iterate over all tasks in the store
    pub fn iter(&self) -> std::slice::Iter<'_, OpenTask> {
        self.open_tasks.iter()
    }

    /// Returns the number of [`Tasks`](Task) in the [`Store`]
    #[must_use]
    pub fn len(&self) -> usize {
        self.open_tasks.len()
    }

    /// Whether the [`Store`] is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn first_missing_id(&self) -> usize {
        let mut ids: Vec<usize> = self.open_tasks.iter().map(|task| task.id).collect();
        ids.sort_unstable();

        ids.iter()
            .enumerate()
            .find(|(idx, id)| idx != *id)
            .map_or(ids.len(), |(idx, _id)| idx)
    }
}

fn load_tasks_from_file(path: impl AsRef<Path>) -> Result<Vec<OpenTask>, Error> {
    let tasks_file = File::options()
        .create(true)
        .write(true)
        .read(true)
        .truncate(false)
        .open(path)?;
    BufReader::new(tasks_file)
        .lines()
        .map(|line| Ok(serde_json::from_str(&line?)?))
        .collect()
}

fn save_tasks_to_file(tasks: &[OpenTask], path: impl AsRef<Path>) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    tasks.iter().try_for_each(|task| {
        serde_json::to_writer(&mut writer, task)?;
        writer.write_all(b"\n")
    })?;

    writer.flush()
}

impl<'a> IntoIterator for &'a Store {
    type IntoIter = std::slice::Iter<'a, OpenTask>;
    type Item = &'a OpenTask;

    fn into_iter(self) -> Self::IntoIter {
        self.open_tasks.iter()
    }
}

/// Error type for store operations
#[derive(Debug, thiserror::Error)]
#[error("Failed to load tasks from file: {0}")]
pub enum Error {
    /// JSON serialization error
    Json(#[from] serde_json::Error),
    /// IO error
    Io(#[from] io::Error),
}

/// A task with an assigned ID for display
///
/// Wraps a [`Task`] and adds a sequential ID field for use in CLI display.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenTask {
    id: usize,
    #[serde(flatten)]
    task: Task,
}

impl OpenTask {
    /// Get the ID of this task
    #[must_use]
    pub const fn id(&self) -> usize {
        self.id
    }

    /// Get a reference to the task
    #[must_use]
    pub const fn task(&self) -> &Task {
        &self.task
    }

    /// Get a mutable reference to the task
    #[must_use]
    pub fn task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
}

impl Deref for OpenTask {
    type Target = Task;

    fn deref(&self) -> &Self::Target {
        &self.task
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::{OpenTask, Store};
    use crate::{store::paths, task::Task};

    #[test]
    fn save_to_file() {
        let dir = TempDir::new().expect("unable to create temporary directory");
        let tasks_file = paths::get_tasks_file(Some(dir.path())).unwrap();

        let mut store = Store::load_from_path(&tasks_file).unwrap();

        store.push(Task::new("some task".to_string()));
        store.push(Task::new("some task".to_string()));
        store.push(Task::new("some task".to_string()));

        store.save_to_path(&tasks_file).unwrap();

        let store2 = Store::load_from_path(&tasks_file).unwrap();

        assert_eq!(store, store2);
    }

    #[test]
    fn missing_id() {
        let mut store = Store::default();

        assert_eq!(store.first_missing_id(), 0);

        store.open_tasks.push(OpenTask {
            id: 0,
            task: Task::new("some task".to_string()),
        });

        assert_eq!(store.first_missing_id(), 1);

        store.open_tasks.push(OpenTask {
            id: 1,
            task: Task::new("some task".to_string()),
        });
        store.open_tasks.push(OpenTask {
            id: 3,
            task: Task::new("some task".to_string()),
        });

        assert_eq!(store.first_missing_id(), 2);
        store.open_tasks.push(OpenTask {
            id: 2,
            task: Task::new("some task".to_string()),
        });

        assert_eq!(store.first_missing_id(), 4);
    }
}
