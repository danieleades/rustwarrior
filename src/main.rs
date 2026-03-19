//! Command-line task tracking

mod cli;
mod commands;
mod store_ext;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::default();
    cli.run()?;

    Ok(())
}
