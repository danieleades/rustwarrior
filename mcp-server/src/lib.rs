//! MCP server for `RustWarrior` task management

mod handler;

use anyhow::Result;
use handler::TaskHandler;
use rmcp::ServiceExt;

/// Run the MCP server over stdio.
///
/// # Errors
///
/// Returns an error if the Tokio runtime cannot be created or if server
/// operations fail.
pub fn run_stdio_server() -> Result<()> {
    initialise_logging();

    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        tracing::info!("Starting RustWarrior MCP server over stdio");
        let handler = TaskHandler::default();

        let service = handler.serve(rmcp::transport::stdio()).await?;
        let quit_reason = service.waiting().await?;
        tracing::info!("Server stopped: {:?}", quit_reason);

        Ok(())
    })
}

fn initialise_logging() {
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();
}
