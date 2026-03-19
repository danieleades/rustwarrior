# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust workspace with a root crate and two supporting crates:

- repository root: the `rw` command-line interface.
- `core/`: shared task model, persistence, and path resolution.
- `mcp-server/`: the MCP library used by the CLI `mcp` subcommand.

The root binary source currently lives under `cli/src/`. Unit tests are colocated with the code under `#[cfg(test)]`. Local task data is stored in `.rustwarrior/` for project-scoped runs, or the platform data directory when `RUSTWARRIOR_DATA_DIR` is unset.

## Build, Test, and Development Commands
- `cargo build --workspace`: build all crates.
- `cargo test --all --all-features`: run the full test suite used by CI.
- `cargo test -p rustwarrior`: run CLI tests only.
- `cargo fmt --all`: format the workspace.
- `cargo clippy --all --all-features --all-targets`: run the same lint scope as CI.
- `cargo run -p rustwarrior -- list`: run the CLI locally.
- `cargo run -p rustwarrior --features mcp -- mcp`: start the MCP server.

## Coding Style & Naming Conventions
Use standard Rust formatting with `cargo fmt`; do not hand-format around it. The workspace denies `clippy::all` and warns on `pedantic` and `nursery`, so keep implementations straightforward and idiomatic. Prefer descriptive snake_case names for functions, modules, and test helpers. Public items should have docs because `missing_docs` is enabled at the workspace level.

## Testing Guidelines
Add focused unit tests next to the code you change. Follow existing naming patterns like `test_get_data_dir_with_env` for core logic and table-driven parsing tests in the CLI. Cover storage path changes, CLI parsing, and persistence edge cases. Before opening a PR, run at least `cargo test --all --all-features` and `cargo fmt --all`.

## Commit & Pull Request Guidelines
Recent history favors short, imperative subjects, often with Conventional Commit prefixes such as `feat:` (`feat: add a model context protocol server`). Keep commits narrowly scoped. PRs should explain the behavioral change, note any storage or CLI impact, link the relevant issue when available, and include example command output for user-facing CLI changes.

## Configuration & Data Tips
`RUSTWARRIOR_DATA_DIR` overrides the default storage location. Example: `RUSTWARRIOR_DATA_DIR=.rustwarrior cargo run -p rustwarrior -- info`. Do not commit generated task data from `.rustwarrior/` or `target/`.

## CLI and MCP Parity
 Treat the CLI and MCP server as parallel user interfaces over the same task functionality. When adding a new CLI feature or command, update `mcp-server/` as well so the capability is exposed through MCP with consistent behavior and storage semantics. If parity is intentionally deferred, call that out explicitly in the PR description.
