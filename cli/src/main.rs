//! Command-line task tracking

#![deny(
    clippy::all,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::cargo_common_metadata)]

mod cli;
mod commands;
mod store_ext;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::default();
    cli.run()?;

    Ok(())
}
