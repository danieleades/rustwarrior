use clap::Parser;
use comfy_table::{presets::UTF8_HORIZONTAL_ONLY, Attribute, Cell, ContentArrangement, Table};
use rustwarrior::Store;

#[derive(Debug, Default, Parser)]
pub struct List {}

impl List {
    pub fn run() -> anyhow::Result<()> {
        let store = Store::load_from_dir_default()?;
        if store.is_empty() {
            println!("no tasks to display");
            return Ok(());
        }
        let mut table = Table::new();
        table
            .load_preset(UTF8_HORIZONTAL_ONLY)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("ID").add_attribute(Attribute::Bold),
                Cell::new("Description").add_attribute(Attribute::Bold),
            ]);

        for task in &store {
            table.add_row(vec![Cell::new(task.id()), Cell::new(task.description())]);
        }

        println!("{table}");

        Ok(())
    }
}
