//! MCP server for `RustWarrior` task management

mod handler;

use anyhow::Result;
use handler::TaskHandler;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<()> {
    initialise_logging();

    tracing::info!("Starting RustWarrior MCP server over stdio");
    let handler = TaskHandler::default();

    let service = handler.serve(rmcp::transport::stdio()).await?;
    let quit_reason = service.waiting().await?;
    tracing::info!("Server stopped: {:?}", quit_reason);

    Ok(())
}

fn initialise_logging() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
}
