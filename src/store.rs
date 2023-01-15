use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    ops::Deref,
    path::{Path, PathBuf},
};

use rustwarrior_core::Task;
use serde::{Deserialize, Serialize};

use crate::APPLICATION_NAME;

const OPEN_TASKS_FILE: &str = "open_tasks.ndjson";

fn default_data_dir() -> io::Result<PathBuf> {
    let dir = dirs::data_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "couldn't find data directory"))?
        .join(APPLICATION_NAME);

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// A collection of [`Tasks`](Task).
///
/// These are represented as [`OpenTasks`](OpenTask), which are simply a wrapper
/// around a [`Task`] that adds a short ID field.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Store {
    open_tasks: Vec<OpenTask>,
}

impl Store {
    fn load_from_dir(path: impl AsRef<Path>) -> Result<Self, Error> {
        let open_tasks = load_tasks_from_file(path.as_ref().join(OPEN_TASKS_FILE))?;

        Ok(Self { open_tasks })
    }

    /// Load the [`Tasks`](Task) from the default location on disk.
    ///
    /// Creates the directory and data file if not present.
    ///
    /// # Errors
    ///
    /// This method can fail if the default data location cannot be determined,
    /// or is not read/writeable.
    pub fn load() -> Result<Self, Error> {
        Self::load_from_dir(default_data_dir()?)
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

    fn save_to_dir(&self, path: impl AsRef<Path>) -> io::Result<()> {
        save_tasks_to_file(&self.open_tasks, path.as_ref().join(OPEN_TASKS_FILE))
    }

    /// Save the [`Tasks`](Task) to the default location on disk.
    ///
    /// Creates the directory and data file if not present.
    ///
    /// # Errors
    ///
    /// This method can fail if the default data location cannot be determined,
    /// or is not writeable.
    pub fn save(&self) -> io::Result<()> {
        self.save_to_dir(default_data_dir()?)
    }

    fn first_missing_id(&self) -> usize {
        let mut ids: Vec<usize> = self.open_tasks.iter().map(OpenTask::id).collect();
        ids.sort_unstable();

        ids.iter()
            .enumerate()
            .find(|(idx, id)| idx != *id)
            .map_or(ids.len(), |(idx, _id)| idx)
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
}

fn load_tasks_from_file(path: impl AsRef<Path>) -> Result<Vec<OpenTask>, Error> {
    let tasks_file = File::options()
        .create(true)
        .write(true)
        .read(true)
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
        writer.write_all(&[b'\n'])
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

#[derive(Debug, thiserror::Error)]
#[error("Failed to load tasks from file: {0}")]
pub enum Error {
    Json(#[from] serde_json::Error),
    Io(#[from] io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenTask {
    id: usize,
    #[serde(flatten)]
    task: Task,
}

impl OpenTask {
    pub const fn id(&self) -> usize {
        self.id
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
    use rustwarrior_core::Task;
    use tempfile::TempDir;

    use crate::store::{OpenTask, Store};

    #[test]
    fn save_to_directory() {
        let dir = TempDir::new().expect("unable to create temporary directory");

        let mut store = Store::load_from_dir(&dir).unwrap();

        store.push(Task::new("some task".to_string()));
        store.push(Task::new("some task".to_string()));
        store.push(Task::new("some task".to_string()));

        store.save_to_dir(&dir).unwrap();

        let store2 = Store::load_from_dir(&dir).unwrap();

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
