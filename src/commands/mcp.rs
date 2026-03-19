use clap::Parser;

/// Run the MCP server over stdio
#[derive(Debug, Default, Parser)]
pub struct Mcp;

impl Mcp {
    /// Run the MCP command
    pub fn run() -> anyhow::Result<()> {
        rustwarrior_mcp_server::run_stdio_server()
    }
}
