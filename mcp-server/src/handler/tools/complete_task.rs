use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::handler::with_store_mut;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CompleteTaskParams {
    pub id: usize,
}

pub fn handle(params: Parameters<CompleteTaskParams>) -> Result<CallToolResult, McpError> {
    let input = params.0;
    let result = with_store_mut(|store| {
        let task = store.get_mut(input.id).ok_or_else(|| {
            McpError::invalid_params(format!("Task {} not found", input.id), None)
        })?;
        task.task_mut().mark_completed();
        Ok(json!({
            "id": input.id,
            "message": "Task marked as completed"
        }))
    })?;

    Ok(CallToolResult {
        content: vec![Content::text(format!("Completed task {}", input.id))],
        structured_content: Some(result),
        is_error: Some(false),
        meta: None,
    })
}
