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
pub struct SetPriorityParams {
    pub id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
}

pub fn handle(params: Parameters<SetPriorityParams>) -> Result<CallToolResult, McpError> {
    let input = params.0;
    let priority = parse_priority(input.priority)?;

    let result = with_store_mut(|store| {
        let task = store.get_mut(input.id).ok_or_else(|| {
            McpError::invalid_params(format!("Task {} not found", input.id), None)
        })?;
        task.task_mut().set_priority(priority);
        Ok(json!({
            "id": input.id,
            "priority": priority.map(u8::from),
            "message": "Priority updated"
        }))
    })?;

    Ok(CallToolResult {
        content: vec![Content::text(format!("Set priority for task {}", input.id))],
        structured_content: Some(result),
        is_error: Some(false),
        meta: None,
    })
}
