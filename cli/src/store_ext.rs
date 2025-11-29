//! CLI-specific convenience methods for Store

use rustwarrior_core::Store;
use std::fs;

/// Extension trait for Store providing default path convenience methods
pub trait StoreExt {
    /// Load tasks from the default location
    fn load_default() -> anyhow::Result<Self>
    where
        Self: Sized;

    /// Save tasks to the default location
    fn save_default(&self) -> anyhow::Result<()>;
}

impl StoreExt for Store {
    fn load_default() -> anyhow::Result<Self> {
        let data_dir = rustwarrior_core::store::paths::get_data_dir()?;
        let tasks_file = rustwarrior_core::store::paths::get_tasks_file(Some(&data_dir))?;
        Self::load_from_path(&tasks_file).map_err(|e| anyhow::anyhow!(e))
    }

    fn save_default(&self) -> anyhow::Result<()> {
        let data_dir = rustwarrior_core::store::paths::get_data_dir()?;
        let tasks_file = rustwarrior_core::store::paths::get_tasks_file(Some(&data_dir))?;
        fs::create_dir_all(&data_dir)?;
        self.save_to_path(&tasks_file).map_err(|e| anyhow::anyhow!(e))
    }
}
