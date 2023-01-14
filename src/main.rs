//! Command-line task tracking

#![deny(
    clippy::all,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
#![warn(clippy::pedantic, clippy::nursery)]

use cli::Cli;

mod cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::default();
    cli.run()?;

    Ok(())
}
