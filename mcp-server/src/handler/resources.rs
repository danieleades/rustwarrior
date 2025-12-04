use chrono::Utc;
use rmcp::model::{Annotated, Annotations, RawResource, Resource, ResourceContents};

pub(super) const DOC_RESOURCE_URI: &str = "rustwarrior://tasks/guide";
pub(super) const DOC_RESOURCE_NAME: &str = "RustWarrior MCP quick guide";
pub(super) const DOC_RESOURCE_TEXT: &str = r"RustWarrior task MCP server

What: lightweight task list for quick notes or per-session context.
When to use: when asked to manage todos or remember personal/project tasks. Good for quick reminders, meeting prep, or small backlogs; not a full project tracker.
Storage: tasks persist in open_tasks.jsonl under RUSTWARRIOR_DATA_DIR or your platform data dir (single-user/local).

Tools
- create_task(description, priority 1-4)
- list_tasks(filter: active | completed | free-text, priority 1-4)
- search_tasks(query, priority 1-4)
- get_task(id)
- set_priority(id, priority 1-4 or null to clear)
- complete_task(id)
- delete_task(id)

Semantics
- Priorities: 1 is highest, 4 is lowest. Invalid values return errors.
- Filters: 'active' hides completed tasks, 'completed' hides active tasks, anything else filters descriptions case-insensitively.
- IDs are zero-based and stable per task.
- Time stamps are UTC strings and may differ from local time.

Typical flow
- create_task -> list_tasks -> set_priority -> complete_task -> delete_task
";

pub(super) const SERVER_INSTRUCTIONS: &str = r"RustWarrior task MCP server for managing todos or personal/project tasks. Use it when the user asks to remember or track tasks.
Tasks are stored locally (RUSTWARRIOR_DATA_DIR or platform data dir).
Tools: create_task(description, priority 1-4), list_tasks(filter=active|completed|text, priority 1-4), search_tasks(query, priority 1-4), get_task(id),
set_priority(id, priority 1-4 or null), complete_task(id), delete_task(id).
Docs: rustwarrior://tasks/guide (what/when/how, filters, priority rules).";

pub(super) fn documentation_resource() -> Resource {
    let mut raw = RawResource::new(DOC_RESOURCE_URI, DOC_RESOURCE_NAME);
    raw.title = Some("RustWarrior task MCP guide".to_string());
    raw.description = Some(
        "What the server does, when to use it, tool parameters, filters, and priority rules."
            .to_string(),
    );
    raw.mime_type = Some("text/plain".to_string());

    let annotations = Some(Annotations::for_resource(0.9, Utc::now()));
    Annotated::new(raw, annotations)
}

pub(super) fn documentation_contents() -> ResourceContents {
    ResourceContents::TextResourceContents {
        uri: DOC_RESOURCE_URI.to_string(),
        mime_type: Some("text/plain".to_string()),
        text: DOC_RESOURCE_TEXT.to_string(),
        meta: None,
    }
}
