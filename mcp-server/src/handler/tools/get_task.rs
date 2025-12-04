use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::handler::{to_task_info, with_store};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetTaskParams {
    pub id: usize,
}

pub fn handle(params: Parameters<GetTaskParams>) -> Result<CallToolResult, McpError> {
    let input = params.0;
    let info = with_store(|store| {
        store
            .get(input.id)
            .map(to_task_info)
            .ok_or_else(|| McpError::invalid_params(format!("Task {} not found", input.id), None))
    })?;

    let data =
        serde_json::to_value(&info).unwrap_or_else(|_| json!({"error": "serialization failed"}));
    Ok(CallToolResult {
        content: vec![Content::text(format!("Task {}", input.id))],
        structured_content: Some(data),
        is_error: Some(false),
        meta: None,
    })
}
