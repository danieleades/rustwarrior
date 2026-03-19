use std::fmt::Write;

use clap::Parser;
use rustwarrior_core::paths::{self, DATA_DIR_ENV_VAR, DataDirSource};

/// Show useful runtime and storage information
#[derive(Debug, Default, Parser)]
pub struct Info;

impl Info {
    /// Run the info command
    pub fn run() -> anyhow::Result<()> {
        print!("{}", Self::render()?);
        Ok(())
    }

    fn render() -> anyhow::Result<String> {
        let info = paths::resolve_storage_info()?;

        let mut output = String::new();
        writeln!(
            output,
            "config file: {}",
            info.config_file
                .as_ref()
                .map_or_else(|| "none".to_string(), |path| path.display().to_string())
        )
        .unwrap();
        match info.config_source {
            DataDirSource::EnvironmentOverride => {
                writeln!(
                    output,
                    "config source: environment variable {DATA_DIR_ENV_VAR}"
                )
                .unwrap();
            }
            DataDirSource::PlatformDefault => {
                writeln!(output, "config source: platform data directory").unwrap();
            }
        }
        writeln!(output, "data dir: {}", info.data_dir.display()).unwrap();
        writeln!(output, "tasks file: {}", info.tasks_file.display()).unwrap();

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use rustwarrior_core::paths::DATA_DIR_ENV_VAR;

    use super::Info;

    #[test]
    fn render_with_env_override() {
        let temp = tempfile::TempDir::new().expect("create temp dir");

        env::set_var(DATA_DIR_ENV_VAR, temp.path());
        let output = Info::render().expect("render info");
        env::remove_var(DATA_DIR_ENV_VAR);

        let tasks_file = PathBuf::from(temp.path()).join("open_tasks.jsonl");

        assert!(output.contains("config file: none"));
        assert!(output.contains(&format!(
            "config source: environment variable {DATA_DIR_ENV_VAR}"
        )));
        assert!(output.contains(&format!("data dir: {}", temp.path().display())));
        assert!(output.contains(&format!("tasks file: {}", tasks_file.display())));
    }
}
