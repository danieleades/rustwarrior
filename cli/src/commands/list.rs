use clap::Parser;
use comfy_table::{presets::UTF8_HORIZONTAL_ONLY, Attribute, Cell, ContentArrangement, Table};
use rustwarrior_core::Store;

use crate::store_ext::StoreExt;

/// List all tasks
#[derive(Debug, Default, Parser)]
pub struct List;

impl List {
    /// Run the list command
    pub fn run() -> anyhow::Result<()> {
        let store = Store::load_default()?;
        if store.is_empty() {
            println!("no tasks to display");
            return Ok(());
        }
        let mut table = Table::new();

        let has_priority = store.into_iter().any(|task| task.priority().is_some());

        let mut header = vec![Cell::new("ID").add_attribute(Attribute::Bold)];
        if has_priority {
            header.push(Cell::new("Priority").add_attribute(Attribute::Bold));
        }
        header.push(Cell::new("Description").add_attribute(Attribute::Bold));

        table
            .load_preset(UTF8_HORIZONTAL_ONLY)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(header);

        for task in &store {
            if has_priority {
                table.add_row(vec![
                    Cell::new(task.id()),
                    Cell::new(
                        task.priority()
                            .map(|priority| priority.to_string())
                            .unwrap_or_default(),
                    ),
                    Cell::new(task.description()),
                ]);
            } else {
                table.add_row(vec![Cell::new(task.id()), Cell::new(task.description())]);
            }
        }

        println!("{table}");

        Ok(())
    }
}
