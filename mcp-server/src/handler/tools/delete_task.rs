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
pub struct DeleteTaskParams {
    pub id: usize,
}

pub fn handle(params: Parameters<DeleteTaskParams>) -> Result<CallToolResult, McpError> {
    let input = params.0;
    let result = with_store_mut(|store| {
        if store.delete(input.id).is_some() {
            Ok(json!({
                "id": input.id,
                "message": "Task deleted successfully"
            }))
        } else {
            Err(McpError::invalid_params(
                format!("Task {} not found", input.id),
                None,
            ))
        }
    })?;

    Ok(CallToolResult {
        content: vec![Content::text(format!("Deleted task {}", input.id))],
        structured_content: Some(result),
        is_error: Some(false),
        meta: None,
    })
}
