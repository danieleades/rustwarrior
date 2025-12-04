# RustWarrior MCP server

A lightweight MCP server that exposes the RustWarrior task list. Use it for quick reminders or session context, not full project tracking.

## Data and IDs
- Tasks persist to `open_tasks.jsonl` in `RUSTWARRIOR_DATA_DIR` (or the platform data dir such as `~/.local/share/rustwarrior`).
- Task IDs are zero-based and remain stable for existing tasks; new tasks take the lowest available ID.

## Tools
- `create_task(description, priority 1-4)` – add a task.
- `list_tasks(filter, priority 1-4)` – `filter` can be `active`, `completed`, or free text (case-insensitive substring on descriptions).
- `search_tasks(query, priority 1-4)` – free-text search on descriptions.
- `get_task(id)` – fetch task details.
- `set_priority(id, priority|null)` – set or clear priority.
- `complete_task(id)` – mark done and timestamp completion.
- `delete_task(id)` – remove a task permanently.

## Semantics
- Priorities: 1 is highest, 4 is lowest. Invalid values return errors.
- Filters: `active` hides completed tasks; `completed` hides active tasks; any other text filters descriptions case-insensitively.
- Time fields are UTC ISO-8601 strings and may differ from local time.
- Invalid IDs return `invalid_params` errors.

## Documentation resource
The server advertises `rustwarrior://tasks/guide` as an MCP resource containing a concise usage guide.

## Typical flow
`create_task` → `list_tasks` → `set_priority` → `complete_task` → `delete_task`
