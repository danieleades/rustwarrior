//! Path resolution for task storage with environment variable override

use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Name of the environment variable that overrides the data directory.
pub const DATA_DIR_ENV_VAR: &str = "RUSTWARRIOR_DATA_DIR";

const APP_DIR: &str = "rustwarrior";
const OPEN_TASKS_FILE: &str = "open_tasks.jsonl";

/// Where the data directory setting came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDirSource {
    /// The directory was set explicitly with [`DATA_DIR_ENV_VAR`].
    EnvironmentOverride,
    /// The directory came from the platform default returned by `dirs`.
    PlatformDefault,
}

/// Resolve the data directory without creating it.
///
/// Respects the [`DATA_DIR_ENV_VAR`] environment variable if set.
/// Otherwise, uses the default platform-specific data directory.
///
/// # Errors
///
/// Returns an error if the platform data directory cannot be determined.
pub fn resolve_data_dir() -> io::Result<(PathBuf, DataDirSource)> {
    if let Some(custom_dir) = std::env::var_os(DATA_DIR_ENV_VAR) {
        return Ok((
            PathBuf::from(custom_dir),
            DataDirSource::EnvironmentOverride,
        ));
    }

    let dir = dirs::data_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "couldn't find data directory"))?
        .join(APP_DIR);

    Ok((dir, DataDirSource::PlatformDefault))
}

/// Get the data directory for storing tasks
///
/// Respects the [`DATA_DIR_ENV_VAR`] environment variable if set.
/// Otherwise, uses the default platform-specific data directory.
///
/// # Errors
///
/// Returns an error if the data directory cannot be determined or created.
pub fn get_data_dir() -> io::Result<PathBuf> {
    let (dir, _source) = resolve_data_dir()?;
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Get the full path to the tasks file
///
/// # Arguments
///
/// * `data_dir` - Optional custom data directory. If None, uses
///   `get_data_dir()`
///
/// # Errors
///
/// Returns an error if the data directory cannot be determined.
pub fn get_tasks_file(data_dir: Option<&Path>) -> io::Result<PathBuf> {
    let dir = if let Some(d) = data_dir {
        d.to_path_buf()
    } else {
        resolve_data_dir()?.0
    };

    Ok(dir.join(OPEN_TASKS_FILE))
}

/// Resolved storage information for `RustWarrior`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageInfo {
    /// Optional config file path. `None` means no config file is used.
    pub config_file: Option<PathBuf>,
    /// Where the active data directory setting came from.
    pub config_source: DataDirSource,
    /// Resolved directory used for persistent task storage.
    pub data_dir: PathBuf,
    /// Resolved path to the task data file.
    pub tasks_file: PathBuf,
}

/// Resolve the active storage configuration without creating directories.
///
/// # Errors
///
/// Returns an error if the data directory cannot be determined.
pub fn resolve_storage_info() -> io::Result<StorageInfo> {
    let (data_dir, config_source) = resolve_data_dir()?;
    let tasks_file = get_tasks_file(Some(&data_dir))?;

    Ok(StorageInfo {
        config_file: None,
        config_source,
        data_dir,
        tasks_file,
    })
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        ffi::OsString,
        path::Path,
        sync::{Mutex, OnceLock},
    };

    use super::*;

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct EnvVarGuard(Option<OsString>);

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(value) = self.0.take() {
                env::set_var(DATA_DIR_ENV_VAR, value);
            } else {
                env::remove_var(DATA_DIR_ENV_VAR);
            }
        }
    }

    fn with_data_dir_env<T>(value: Option<&Path>, f: impl FnOnce() -> T) -> T {
        let _lock = env_lock().lock().unwrap();
        let _guard = EnvVarGuard(env::var_os(DATA_DIR_ENV_VAR));

        if let Some(path) = value {
            env::set_var(DATA_DIR_ENV_VAR, path);
        } else {
            env::remove_var(DATA_DIR_ENV_VAR);
        }

        f()
    }

    #[test]
    fn test_get_data_dir_default() {
        let result = with_data_dir_env(None, get_data_dir);
        assert!(result.is_ok());
        let dir = result.unwrap();
        assert!(dir.ends_with(APP_DIR));
    }

    #[test]
    fn test_get_data_dir_with_env() {
        let temp = tempfile::TempDir::new().expect("create temp dir");
        let path = temp.path().to_str().unwrap().to_string();

        let result = with_data_dir_env(Some(temp.path()), get_data_dir);
        assert!(result.is_ok());
        let dir = result.unwrap();
        assert_eq!(dir, PathBuf::from(&path));
    }

    #[test]
    fn test_resolve_data_dir_with_env() {
        let temp = tempfile::TempDir::new().expect("create temp dir");
        let path = temp.path().to_str().unwrap().to_string();

        let result = with_data_dir_env(Some(temp.path()), resolve_data_dir);
        assert!(result.is_ok());
        let (dir, source) = result.unwrap();
        assert_eq!(dir, PathBuf::from(&path));
        assert_eq!(source, DataDirSource::EnvironmentOverride);
    }

    #[test]
    fn test_get_tasks_file() {
        let temp = tempfile::TempDir::new().expect("create temp dir");
        let result = get_tasks_file(Some(temp.path()));
        assert!(result.is_ok());
        let file = result.unwrap();
        assert!(file.ends_with("open_tasks.jsonl"));
    }

    #[test]
    fn test_resolve_storage_info_with_env() {
        let temp = tempfile::TempDir::new().expect("create temp dir");

        let result = with_data_dir_env(Some(temp.path()), resolve_storage_info);
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.config_file, None);
        assert_eq!(info.config_source, DataDirSource::EnvironmentOverride);
        assert_eq!(info.data_dir, temp.path());
        assert_eq!(info.tasks_file, temp.path().join(OPEN_TASKS_FILE));
    }

    #[cfg(unix)]
    #[test]
    fn test_resolve_data_dir_with_non_utf8_env() {
        use std::os::unix::ffi::OsStringExt;

        let path = PathBuf::from(OsString::from_vec(vec![b'/', b't', b'm', b'p', 0xFF]));

        let result = with_data_dir_env(Some(&path), resolve_data_dir);
        assert!(result.is_ok());
        let (dir, source) = result.unwrap();
        assert_eq!(dir, path);
        assert_eq!(source, DataDirSource::EnvironmentOverride);
    }
}
