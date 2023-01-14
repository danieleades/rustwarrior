use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::task::Task;

const OPEN_TASKS_FILE: &str = "open_tasks.ndjson";

fn default_data_dir() -> io::Result<PathBuf> {
    let dir = dirs::data_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "couldn't find data directory"))?
        .join("rustwarrior");

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Store {
    open_tasks: Vec<Task>,
}

impl Store {
    pub fn load_from_dir(path: impl AsRef<Path>) -> Result<Self, Error> {
        let open_tasks = load_tasks_from_file(path.as_ref().join(OPEN_TASKS_FILE))?;

        Ok(Self { open_tasks })
    }

    pub fn load_from_dir_default() -> Result<Self, Error> {
        Self::load_from_dir(default_data_dir()?)
    }

    pub fn push(&mut self, task: Task) {
        self.open_tasks.push(task);
    }

    pub fn save_to_dir(&self, path: impl AsRef<Path>) -> io::Result<()> {
        save_tasks_to_file(&self.open_tasks, path.as_ref().join(OPEN_TASKS_FILE))
    }

    pub fn save_to_dir_default(&self) -> io::Result<()> {
        self.save_to_dir(default_data_dir()?)
    }

    /// Return the lowest ID in the list of tasks which is not already used
    ///
    /// # Example
    ///
    /// ```
    /// use rustwarrior::{Store, Task};
    ///
    /// let mut store = Store::default();
    ///
    /// assert_eq!(store.first_missing_id(), 0);
    ///
    /// store.push(Task::new(0, "some task".to_string()));
    /// store.push(Task::new(1, "some task".to_string()));
    /// store.push(Task::new(3, "some task".to_string()));
    ///
    /// assert_eq!(store.first_missing_id(), 2);
    /// ```
    pub fn first_missing_id(&self) -> usize {
        let mut ids: Vec<usize> = self.open_tasks.iter().map(Task::id).collect();
        ids.sort_unstable();

        ids.iter()
            .enumerate()
            .find(|(idx, id)| idx != *id)
            .map_or(ids.len(), |(idx, _id)| idx)
    }

    pub fn len(&self) -> usize {
        self.open_tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn load_tasks_from_file(path: impl AsRef<Path>) -> Result<Vec<Task>, Error> {
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

fn save_tasks_to_file(tasks: &[Task], path: impl AsRef<Path>) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    tasks.iter().try_for_each(|task| {
        serde_json::to_writer(&mut writer, task)?;
        writer.write_all(&[b'\n'])
    })?;

    writer.flush()
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to load tasks from file: {0}")]
pub enum Error {
    Json(#[from] serde_json::Error),
    Io(#[from] io::Error),
}

impl<'a> IntoIterator for &'a Store {
    type IntoIter = std::slice::Iter<'a, Task>;
    type Item = &'a Task;

    fn into_iter(self) -> Self::IntoIter {
        self.open_tasks.iter()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::Store;
    use crate::task::Task;

    #[test]
    fn save_to_directory() {
        let dir = TempDir::new().expect("unable to create temporary directory");

        let mut store = Store::load_from_dir(&dir).unwrap();

        store.push(Task::new(1, "some task".to_string()));
        store.push(Task::new(2, "some task".to_string()));
        store.push(Task::new(3, "some task".to_string()));

        store.save_to_dir(&dir).unwrap();

        let store2 = Store::load_from_dir(&dir).unwrap();

        assert_eq!(store, store2);
    }

    #[test]
    fn missing_id() {
        let mut store = Store::default();

        assert_eq!(store.first_missing_id(), 0);

        store.push(Task::new(0, "some task".to_string()));

        assert_eq!(store.first_missing_id(), 1);

        store.push(Task::new(1, "some task".to_string()));
        store.push(Task::new(3, "some task".to_string()));

        assert_eq!(store.first_missing_id(), 2);

        store.push(Task::new(2, "some task".to_string()));

        assert_eq!(store.first_missing_id(), 4);
    }
}
