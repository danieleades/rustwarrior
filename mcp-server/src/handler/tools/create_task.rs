use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::handler::{parse_priority, with_store_mut};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateTaskParams {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
}

pub fn handle(params: Parameters<CreateTaskParams>) -> Result<CallToolResult, McpError> {
    let input = params.0;
    let priority = parse_priority(input.priority)?;

    let result = with_store_mut(|store| {
        let mut task = rustwarrior_core::Task::new(input.description);
        if let Some(p) = priority {
            task.set_priority(Some(p));
        }
        let id = store.push(task);
        Ok(json!({
            "id": id,
            "message": "Task created successfully"
        }))
    })?;

    Ok(CallToolResult {
        content: vec![Content::text(format!("Created task {}", result["id"]))],
        structured_content: Some(result),
        is_error: Some(false),
        meta: None,
    })
}
