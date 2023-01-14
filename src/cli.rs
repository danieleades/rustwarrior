use clap::Parser;

use self::{add::Add, list::List};

mod add;
mod list;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        self.command.unwrap_or_default().run()
    }
}

#[derive(Debug, Parser)]
pub enum Command {
    Add(Add),
    List,
}

impl Default for Command {
    fn default() -> Self {
        Self::List
    }
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Add(add) => add.run(),
            Self::List => List::run(),
        }
    }
}
