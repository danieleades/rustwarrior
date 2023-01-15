use clap::Parser;
use comfy_table::{presets::UTF8_HORIZONTAL_ONLY, Attribute, Cell, ContentArrangement, Table};
use rustwarrior::Store;

use crate::config::Config;

#[derive(Debug, Default, Parser)]
pub struct List {}

impl List {
    pub fn run() -> anyhow::Result<()> {
        let store = Store::load()?;
        if store.is_empty() {
            println!("no tasks to display");
            return Ok(());
        }

        let config = Config::load()?;

        let mut tasks: Vec<_> = (&store).into_iter().collect();
        tasks.sort_by(|t1, t2| {
            t2.urgency(&config.coefficients)
                .total_cmp(&t1.urgency(&config.coefficients))
        });

        let mut table = Table::new();

        let has_priority = tasks.iter().any(|task| task.priority().is_some());

        let mut header = vec![Cell::new("ID").add_attribute(Attribute::Bold)];
        if has_priority {
            header.push(Cell::new("Priority").add_attribute(Attribute::Bold));
        }
        header.push(Cell::new("Description").add_attribute(Attribute::Bold));
        header.push(Cell::new("Urgency").add_attribute(Attribute::Bold));

        table
            .load_preset(UTF8_HORIZONTAL_ONLY)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(header);

        for task in tasks {
            let mut row = vec![Cell::new(task.id())];
            if has_priority {
                row.push(Cell::new(
                    task.priority()
                        .map(|priority| priority.to_string())
                        .unwrap_or_default(),
                ));
            }
            row.push(Cell::new(task.description()));
            row.push(Cell::new(task.urgency(&config.coefficients)));
            table.add_row(row);
        }

        println!("{table}");

        Ok(())
    }
}
