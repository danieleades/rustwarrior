use clap::Parser;
use comfy_table::{Attribute, Cell, ContentArrangement, Table, presets::UTF8_HORIZONTAL_ONLY};
use rustwarrior_core::{OpenTask, Project, Store};

use crate::store_ext::StoreExt;

/// List all tasks
#[derive(Debug, Default, Parser)]
pub struct List {
    /// Filter tasks by project or parent project
    #[clap(long)]
    project: Option<Project>,
}

impl List {
    /// Run the list command
    pub fn run(self) -> anyhow::Result<()> {
        let store = Store::load_default()?;
        let tasks: Vec<&OpenTask> = store
            .iter()
            .filter(|task| {
                self.project
                    .as_ref()
                    .is_none_or(|project| task.matches_project(project))
            })
            .collect();

        if tasks.is_empty() {
            println!("no tasks to display");
            return Ok(());
        }
        let mut table = Table::new();

        let has_priority = tasks.iter().any(|task| task.priority().is_some());
        let has_project = tasks.iter().any(|task| task.project().is_some());

        let mut header = vec![Cell::new("ID").add_attribute(Attribute::Bold)];
        if has_priority {
            header.push(Cell::new("Priority").add_attribute(Attribute::Bold));
        }
        if has_project {
            header.push(Cell::new("Project").add_attribute(Attribute::Bold));
        }
        header.push(Cell::new("Description").add_attribute(Attribute::Bold));

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
            if has_project {
                row.push(Cell::new(
                    task.project().map(Project::as_str).unwrap_or_default(),
                ));
            }
            row.push(Cell::new(task.description()));
            table.add_row(row);
        }

        println!("{table}");

        Ok(())
    }
}
