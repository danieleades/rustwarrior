//! Path resolution for task storage with environment variable override

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const OPEN_TASKS_FILE: &str = "open_tasks.ndjson";

/// Get the data directory for storing tasks
///
/// Respects the `RUSTWARRIOR_DATA_DIR` environment variable if set.
/// Otherwise, uses the default platform-specific data directory.
///
/// # Errors
///
/// Returns an error if the data directory cannot be determined or created.
pub fn get_data_dir() -> io::Result<PathBuf> {
    if let Ok(custom_dir) = std::env::var("RUSTWARRIOR_DATA_DIR") {
        let path = PathBuf::from(custom_dir);
        fs::create_dir_all(&path)?;
        return Ok(path);
    }

    let dir = dirs::data_dir()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "couldn't find data directory",
            )
        })?
        .join("rustwarrior");

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Get the full path to the tasks file
///
/// # Arguments
///
/// * `data_dir` - Optional custom data directory. If None, uses `get_data_dir()`
///
/// # Errors
///
/// Returns an error if the data directory cannot be determined.
pub fn get_tasks_file(data_dir: Option<&Path>) -> io::Result<PathBuf> {
    let dir = if let Some(d) = data_dir {
        d.to_path_buf()
    } else {
        get_data_dir()?
    };

    Ok(dir.join(OPEN_TASKS_FILE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_data_dir_default() {
        // Clear env var if set
        env::remove_var("RUSTWARRIOR_DATA_DIR");

        let result = get_data_dir();
        assert!(result.is_ok());
        let dir = result.unwrap();
        assert!(dir.ends_with("rustwarrior"));
    }

    #[test]
    fn test_get_data_dir_with_env() {
        let temp = tempfile::TempDir::new().expect("create temp dir");
        let path = temp.path().to_str().unwrap().to_string();

        env::set_var("RUSTWARRIOR_DATA_DIR", &path);
        let result = get_data_dir();
        env::remove_var("RUSTWARRIOR_DATA_DIR");

        assert!(result.is_ok());
        let dir = result.unwrap();
        assert_eq!(dir, PathBuf::from(&path));
    }

    #[test]
    fn test_get_tasks_file() {
        let temp = tempfile::TempDir::new().expect("create temp dir");
        let result = get_tasks_file(Some(temp.path()));
        assert!(result.is_ok());
        let file = result.unwrap();
        assert!(file.ends_with("open_tasks.ndjson"));
    }
}
