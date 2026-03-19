use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rustwarrior_core::paths::{self, DATA_DIR_ENV_VAR, DataDirSource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct InfoParams {}

pub fn handle(_params: Parameters<InfoParams>) -> Result<CallToolResult, McpError> {
    let info = paths::resolve_storage_info().map_err(|e| {
        McpError::internal_error(format!("Failed to resolve storage information: {e}"), None)
    })?;

    let config_source = match info.config_source {
        DataDirSource::EnvironmentOverride => {
            format!("environment variable {DATA_DIR_ENV_VAR}")
        }
        DataDirSource::PlatformDefault => "platform data directory".to_string(),
    };

    let data = json!({
        "config_file": info
            .config_file
            .as_ref()
            .map(|path| path.display().to_string()),
        "config_source": config_source,
        "data_dir": info.data_dir.display().to_string(),
        "tasks_file": info.tasks_file.display().to_string(),
    });

    Ok(CallToolResult {
        content: vec![Content::text(format!(
            "RustWarrior storage: {}",
            info.tasks_file.display()
        ))],
        structured_content: Some(data),
        is_error: Some(false),
        meta: None,
    })
}

#[cfg(test)]
mod tests {
    use std::env;

    use rmcp::handler::server::wrapper::Parameters;
    use rustwarrior_core::paths::DATA_DIR_ENV_VAR;

    use super::{InfoParams, handle};

    #[test]
    fn returns_storage_info() {
        let temp = tempfile::TempDir::new().expect("create temp dir");

        env::set_var(DATA_DIR_ENV_VAR, temp.path());
        let result = handle(Parameters(InfoParams::default())).expect("info tool result");
        env::remove_var(DATA_DIR_ENV_VAR);

        let content = result
            .structured_content
            .expect("structured content should be present");

        assert_eq!(content["config_file"], serde_json::Value::Null);
        assert_eq!(
            content["config_source"],
            format!("environment variable {DATA_DIR_ENV_VAR}")
        );
        assert_eq!(content["data_dir"], temp.path().display().to_string());
        assert_eq!(
            content["tasks_file"],
            temp.path().join("open_tasks.jsonl").display().to_string()
        );
    }
}
