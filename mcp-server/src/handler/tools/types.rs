use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct TaskInfo {
    pub id: usize,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    pub created: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    pub is_completed: bool,
}
