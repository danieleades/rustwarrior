//! Task management MCP server implementation

use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler,
};
use rustwarrior_core::{Priority, Store};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Task management server handler
#[derive(Clone)]
pub struct TaskHandler {
    /// Generated router containing all exposed tools.
    pub(crate) tool_router: ToolRouter<Self>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateTaskParams {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListTasksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
}

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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetTaskParams {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CompleteTaskParams {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteTaskParams {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetPriorityParams {
    pub id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchTasksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
}

impl TaskHandler {
    /// Create a new server handler
    #[must_use]
    pub fn new() -> Self {
        Self {
            tool_router: Self::build_tool_router(),
        }
    }

    pub(crate) fn build_tool_router() -> ToolRouter<Self> {
        Self::tool_router()
    }
}

impl Default for TaskHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler]
impl ServerHandler for TaskHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(
                "RustWarrior task management server. Use create_task to add tasks, \
                 list_tasks to view them, complete_task to mark done, delete_task to remove, \
                 set_priority to change priority (1-4), search_tasks to find tasks by \
                 description, and get_task to view a specific task."
                    .to_owned(),
            ),
            ..ServerInfo::default()
        }
    }
}

#[tool_router]
impl TaskHandler {
    #[tool(
        description = "Create a new task with optional priority",
        annotations(read_only_hint = false, idempotent_hint = false)
    )]
    async fn create_task(
        &self,
        params: Parameters<CreateTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        let input = params.0;
        let priority = if let Some(p) = input.priority {
            Priority::try_from(p).ok()
        } else {
            None
        };

        match load_and_save_store(|store| {
            let mut task = rustwarrior_core::Task::new(input.description);
            if let Some(p) = priority {
                task.set_priority(Some(p));
            }
            let id = store.push(task);
            Ok(json!({
                "id": id,
                "message": "Task created successfully"
            }))
        }) {
            Ok(result) => Ok(CallToolResult {
                content: vec![Content::text(format!("Created task {}", result["id"]))],
                structured_content: Some(result),
                is_error: Some(false),
                meta: None,
            }),
            Err(e) => Err(McpError::internal_error(
                format!("Failed to create task: {}", e),
                None,
            )),
        }
    }

    #[tool(
        description = "List all tasks with optional filters",
        annotations(read_only_hint = true, idempotent_hint = true)
    )]
    async fn list_tasks(
        &self,
        params: Parameters<ListTasksParams>,
    ) -> Result<CallToolResult, McpError> {
        let input = params.0;
        match load_store() {
            Ok(store) => {
                let priority_filter = if let Some(p) = input.priority {
                    Priority::try_from(p).ok()
                } else {
                    None
                };

                let tasks: Vec<TaskInfo> = store
                    .iter()
                    .filter(|task| {
                        if let Some(f) = &input.filter {
                            match f.as_str() {
                                "active" if task.is_completed() => return false,
                                "completed" if !task.is_completed() => return false,
                                _ => {}
                            }
                        }

                        if let Some(pf) = priority_filter {
                            if task.priority() != Some(pf) {
                                return false;
                            }
                        }

                        true
                    })
                    .map(|task| TaskInfo {
                        id: task.id(),
                        description: task.description().to_string(),
                        priority: task.priority().map(|p| p as u8),
                        created: task.created().to_string(),
                        completed: task.completed().map(|c| c.to_string()),
                        is_completed: task.is_completed(),
                    })
                    .collect();

                let data = json!({ "tasks": tasks });
                Ok(CallToolResult {
                    content: vec![Content::text(format!("Listed {} tasks", tasks.len()))],
                    structured_content: Some(data),
                    is_error: Some(false),
                    meta: None,
                })
            }
            Err(e) => Err(McpError::internal_error(
                format!("Failed to load tasks: {}", e),
                None,
            )),
        }
    }

    #[tool(
        description = "Get a specific task by ID",
        annotations(read_only_hint = true, idempotent_hint = true)
    )]
    async fn get_task(
        &self,
        params: Parameters<GetTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        let input = params.0;
        match load_store() {
            Ok(store) => match store.get(input.id) {
                Some(task) => {
                    let info = TaskInfo {
                        id: task.id(),
                        description: task.description().to_string(),
                        priority: task.priority().map(|p| p as u8),
                        created: task.created().to_string(),
                        completed: task.completed().map(|c| c.to_string()),
                        is_completed: task.is_completed(),
                    };
                    let data = serde_json::to_value(&info)
                        .unwrap_or_else(|_| json!({"error": "serialization failed"}));
                    Ok(CallToolResult {
                        content: vec![Content::text(format!("Task {}", input.id))],
                        structured_content: Some(data),
                        is_error: Some(false),
                        meta: None,
                    })
                }
                None => Err(McpError::invalid_params(
                    format!("Task {} not found", input.id),
                    None,
                )),
            },
            Err(e) => Err(McpError::internal_error(
                format!("Failed to load tasks: {}", e),
                None,
            )),
        }
    }

    #[tool(
        description = "Mark a task as completed",
        annotations(read_only_hint = false, idempotent_hint = false)
    )]
    async fn complete_task(
        &self,
        params: Parameters<CompleteTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        let input = params.0;
        match load_and_save_store(|store| {
            if let Some(task_ref) = store.get(input.id) {
                let mut task = task_ref.task().clone();
                task.mark_completed();
                store.delete(input.id);
                store.push(task);
                Ok(json!({
                    "id": input.id,
                    "message": "Task marked as completed"
                }))
            } else {
                Err(format!("Task {} not found", input.id))
            }
        }) {
            Ok(result) => Ok(CallToolResult {
                content: vec![Content::text(format!("Completed task {}", input.id))],
                structured_content: Some(result),
                is_error: Some(false),
                meta: None,
            }),
            Err(e) => Err(McpError::invalid_params(format!("Failed to complete task: {}", e), None)),
        }
    }

    #[tool(
        description = "Delete a task",
        annotations(read_only_hint = false, idempotent_hint = false)
    )]
    async fn delete_task(
        &self,
        params: Parameters<DeleteTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        let input = params.0;
        match load_and_save_store(|store| {
            if store.delete(input.id).is_some() {
                Ok(json!({
                    "id": input.id,
                    "message": "Task deleted successfully"
                }))
            } else {
                Err(format!("Task {} not found", input.id))
            }
        }) {
            Ok(result) => Ok(CallToolResult {
                content: vec![Content::text(format!("Deleted task {}", input.id))],
                structured_content: Some(result),
                is_error: Some(false),
                meta: None,
            }),
            Err(e) => Err(McpError::invalid_params(format!("Failed to delete task: {}", e), None)),
        }
    }

    #[tool(
        description = "Set the priority of a task",
        annotations(read_only_hint = false, idempotent_hint = false)
    )]
    async fn set_priority(
        &self,
        params: Parameters<SetPriorityParams>,
    ) -> Result<CallToolResult, McpError> {
        let input = params.0;
        let priority = if let Some(p) = input.priority {
            Priority::try_from(p).ok()
        } else {
            None
        };

        match load_and_save_store(|store| {
            if let Some(task_ref) = store.get(input.id) {
                let mut task = task_ref.task().clone();
                task.set_priority(priority);
                store.delete(input.id);
                store.push(task);
                Ok(json!({
                    "id": input.id,
                    "priority": priority.map(|p| p as u8),
                    "message": "Priority updated"
                }))
            } else {
                Err(format!("Task {} not found", input.id))
            }
        }) {
            Ok(result) => Ok(CallToolResult {
                content: vec![Content::text(format!("Set priority for task {}", input.id))],
                structured_content: Some(result),
                is_error: Some(false),
                meta: None,
            }),
            Err(e) => Err(McpError::invalid_params(format!("Failed to set priority: {e}"), None)),
        }
    }

    #[tool(
        description = "Search tasks by description and/or priority",
        annotations(read_only_hint = true, idempotent_hint = true)
    )]
    async fn search_tasks(
        &self,
        params: Parameters<SearchTasksParams>,
    ) -> Result<CallToolResult, McpError> {
        let input = params.0;
        match load_store() {
            Ok(store) => {
                let priority_filter = if let Some(p) = input.priority {
                    Priority::try_from(p).ok()
                } else {
                    None
                };

                let tasks: Vec<TaskInfo> = store
                    .iter()
                    .filter(|task| {
                        if let Some(q) = &input.query {
                            if !task
                                .description()
                                .to_lowercase()
                                .contains(&q.to_lowercase())
                            {
                                return false;
                            }
                        }

                        if let Some(pf) = priority_filter {
                            if task.priority() != Some(pf) {
                                return false;
                            }
                        }

                        true
                    })
                    .map(|task| TaskInfo {
                        id: task.id(),
                        description: task.description().to_string(),
                        priority: task.priority().map(|p| p as u8),
                        created: task.created().to_string(),
                        completed: task.completed().map(|c| c.to_string()),
                        is_completed: task.is_completed(),
                    })
                    .collect();

                let data = json!({ "tasks": tasks });
                Ok(CallToolResult {
                    content: vec![Content::text(format!("Found {} tasks", tasks.len()))],
                    structured_content: Some(data),
                    is_error: Some(false),
                    meta: None,
                })
            }
            Err(e) => Err(McpError::internal_error(
                format!("Failed to search tasks: {}", e),
                None,
            )),
        }
    }
}

fn load_store() -> Result<Store, Box<dyn std::error::Error>> {
    let data_dir = rustwarrior_core::store::paths::get_data_dir()?;
    let tasks_file = rustwarrior_core::store::paths::get_tasks_file(Some(&data_dir))?;
    Store::load_from_path(&tasks_file).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn load_and_save_store<F, T>(f: F) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnOnce(&mut Store) -> Result<T, String>,
{
    let data_dir = rustwarrior_core::store::paths::get_data_dir()?;
    let tasks_file = rustwarrior_core::store::paths::get_tasks_file(Some(&data_dir))?;

    let mut store = Store::load_from_path(&tasks_file)?;
    let result = f(&mut store)?;

    std::fs::create_dir_all(&data_dir)?;
    store.save_to_path(&tasks_file)?;

    Ok(result)
}
