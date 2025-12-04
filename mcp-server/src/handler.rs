//! Task management MCP server implementation

mod resources;
mod tools;

use std::{future::Future, path::PathBuf};

use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, ListResourcesResult, PaginatedRequestParam, ReadResourceRequestParam,
        ReadResourceResult, ServerCapabilities, ServerInfo,
    },
    service::{RequestContext, RoleServer},
    tool, tool_handler, tool_router,
};
use rustwarrior_core::{OpenTask, Priority, Store};

use self::{
    resources::{
        DOC_RESOURCE_URI, SERVER_INSTRUCTIONS, documentation_contents, documentation_resource,
    },
    tools::TaskInfo,
};

/// Task management server handler
#[derive(Clone)]
pub struct TaskHandler {
    /// Generated router containing all exposed tools.
    pub(crate) tool_router: ToolRouter<Self>,
}

impl Default for TaskHandler {
    fn default() -> Self {
        Self {
            tool_router: Self::tool_router(),
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
        params: Parameters<tools::create_task::CreateTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::create_task::handle(params)
    }

    #[tool(
        description = "List all tasks with optional filters",
        annotations(read_only_hint = true, idempotent_hint = true)
    )]
    async fn list_tasks(
        &self,
        params: Parameters<tools::list_tasks::ListTasksParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::list_tasks::handle(params)
    }

    #[tool(
        description = "Get a specific task by ID",
        annotations(read_only_hint = true, idempotent_hint = true)
    )]
    async fn get_task(
        &self,
        params: Parameters<tools::get_task::GetTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::get_task::handle(params)
    }

    #[tool(
        description = "Mark a task as completed",
        annotations(read_only_hint = false, idempotent_hint = false)
    )]
    async fn complete_task(
        &self,
        params: Parameters<tools::complete_task::CompleteTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::complete_task::handle(params)
    }

    #[tool(
        description = "Delete a task",
        annotations(read_only_hint = false, idempotent_hint = false)
    )]
    async fn delete_task(
        &self,
        params: Parameters<tools::delete_task::DeleteTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::delete_task::handle(params)
    }

    #[tool(
        description = "Set the priority of a task",
        annotations(read_only_hint = false, idempotent_hint = false)
    )]
    async fn set_priority(
        &self,
        params: Parameters<tools::set_priority::SetPriorityParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::set_priority::handle(params)
    }

    #[tool(
        description = "Search tasks by description and/or priority",
        annotations(read_only_hint = true, idempotent_hint = true)
    )]
    async fn search_tasks(
        &self,
        params: Parameters<tools::search_tasks::SearchTasksParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::search_tasks::handle(params)
    }
}

#[tool_handler]
impl ServerHandler for TaskHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
            instructions: Some(SERVER_INSTRUCTIONS.to_owned()),
            ..ServerInfo::default()
        }
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListResourcesResult, McpError>> + Send + '_ {
        std::future::ready(Ok(ListResourcesResult::with_all_items(vec![
            documentation_resource(),
        ])))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
        std::future::ready(if request.uri == DOC_RESOURCE_URI {
            Ok(ReadResourceResult {
                contents: vec![documentation_contents()],
            })
        } else {
            Err(McpError::invalid_params(
                format!("Unknown resource {}", request.uri),
                None,
            ))
        })
    }
}

fn store_paths() -> Result<(PathBuf, PathBuf), McpError> {
    let data_dir = rustwarrior_core::store::paths::get_data_dir().map_err(|e| {
        McpError::internal_error(format!("Failed to resolve data directory: {e}"), None)
    })?;
    let tasks_file =
        rustwarrior_core::store::paths::get_tasks_file(Some(&data_dir)).map_err(|e| {
            McpError::internal_error(format!("Failed to resolve tasks file: {e}"), None)
        })?;
    Ok((data_dir, tasks_file))
}

fn load_store_with_paths() -> Result<(Store, PathBuf, PathBuf), McpError> {
    let (data_dir, tasks_file) = store_paths()?;
    let store = Store::load_from_path(&tasks_file)
        .map_err(|e| McpError::internal_error(format!("Failed to load tasks: {e}"), None))?;
    Ok((store, data_dir, tasks_file))
}

pub fn with_store<F, T>(f: F) -> Result<T, McpError>
where
    F: FnOnce(&Store) -> Result<T, McpError>,
{
    let (store, _, _) = load_store_with_paths()?;
    f(&store)
}

pub fn with_store_mut<F, T>(f: F) -> Result<T, McpError>
where
    F: FnOnce(&mut Store) -> Result<T, McpError>,
{
    let (mut store, data_dir, tasks_file) = load_store_with_paths()?;
    let output = f(&mut store)?;

    std::fs::create_dir_all(&data_dir).map_err(|e| {
        McpError::internal_error(format!("Failed to prepare data directory: {e}"), None)
    })?;
    store
        .save_to_path(&tasks_file)
        .map_err(|e| McpError::internal_error(format!("Failed to save tasks: {e}"), None))?;

    Ok(output)
}

pub fn parse_priority(input: Option<u8>) -> Result<Option<Priority>, McpError> {
    input
        .map(Priority::try_from)
        .transpose()
        .map_err(|e| McpError::invalid_params(e.to_string(), None))
}

pub fn normalize_filter(input: Option<&String>) -> Option<String> {
    input
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_lowercase)
}

pub fn to_task_info(task: &OpenTask) -> TaskInfo {
    TaskInfo {
        id: task.id(),
        description: task.description().clone(),
        priority: task.priority().map(u8::from),
        created: task.created().to_string(),
        completed: task.completed().map(|c| c.to_string()),
        is_completed: task.is_completed(),
    }
}
