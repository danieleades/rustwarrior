use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::TaskInfo;
use crate::handler::{
    matches_priority_and_project, normalize_filter, parse_priority, parse_project, to_task_info,
    with_store,
};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchTasksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

pub fn handle(params: Parameters<SearchTasksParams>) -> Result<CallToolResult, McpError> {
    let input = params.0;
    let priority_filter = parse_priority(input.priority)?;
    let project_filter = parse_project(input.project)?;
    let query = normalize_filter(input.query.as_deref());

    let tasks: Vec<TaskInfo> = with_store(|store| {
        let tasks = store
            .iter()
            .filter(|task| {
                if let Some(query_text) = query.as_ref() {
                    if !task.description().to_lowercase().contains(query_text) {
                        return false;
                    }
                }

                matches_priority_and_project(task, priority_filter, project_filter.as_ref())
            })
            .map(to_task_info)
            .collect();
        Ok(tasks)
    })?;

    let data = json!({ "tasks": tasks });
    Ok(CallToolResult {
        content: vec![Content::text(format!("Found {} tasks", tasks.len()))],
        structured_content: Some(data),
        is_error: Some(false),
        meta: None,
    })
}
