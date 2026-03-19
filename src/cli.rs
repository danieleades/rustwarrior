use clap::Parser;

#[cfg(feature = "mcp")]
use crate::commands::mcp::Mcp;
use crate::commands::{add::Add, info::Info, list::List};

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
    Info,
    List(List),
    #[cfg(feature = "mcp")]
    Mcp,
}

impl Default for Command {
    fn default() -> Self {
        Self::List(List::default())
    }
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Add(add) => add.run(),
            Self::Info => Info::run(),
            Self::List(list) => list.run(),
            #[cfg(feature = "mcp")]
            Self::Mcp => Mcp::run(),
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use test_case::test_case;

    use super::Cli;

    #[test_case("rw" ; "empty")]
    #[test_case("rw list" ; "list")]
    #[test_case("rw list --project Home.Garden" ; "list project")]
    #[test_case("rw info")]
    #[test_case(r#"rw add "some new task""# ; "add")]
    #[test_case(r#"rw add "some new task" -p 1"# ; "priority short")]
    #[test_case(r#"rw add "some new task" --priority 1"# ; "priority long")]
    #[test_case(r#"rw add "some new task" --priority=1"# ; "priority long alt")]
    #[test_case(r#"rw add "some new task" --project Home.Garden"# ; "project")]
    fn parse(input: &str) {
        Cli::parse_from(shlex::split(input).unwrap());
    }

    #[cfg(feature = "mcp")]
    #[test]
    fn parse_mcp() {
        Cli::parse_from(shlex::split("rw mcp").unwrap());
    }
}
