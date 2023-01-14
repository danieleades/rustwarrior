#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

use cli::Cli;

mod cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::default();
    cli.run()?;

    Ok(())
}
