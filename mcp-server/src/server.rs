//! `RustWarrior` MCP server implementation

use crate::handler::TaskHandler;
use rmcp::Server;

/// Run the MCP server
///
/// # Errors
///
/// Returns an error if server operations fail.
pub async fn run_server() -> anyhow::Result<()> {
    let handler = TaskHandler;
    let server = Server::new(handler);

    server
        .serve(
            tokio::io::stdin(),
            tokio::io::stdout(),
            Default::default(),
        )
        .await?;

    Ok(())
}
