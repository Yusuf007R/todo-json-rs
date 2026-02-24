use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};
mod cli;
mod model;
mod ui;
use crate::cli::{Cli, Commands};

mod storage;
use crate::storage::Storage;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut db = Storage::load()?;

    let changed: bool = match cli.command {
        Commands::Add { content } => {
            db.add_todo(content.join(" "));
            true
        }

        Commands::Ls => {
            let max_id = db
                .iter()
                .map(|t| t.id().to_string().len())
                .max()
                .unwrap_or(0);
            let max_content = db.iter().map(|t| t.content().len()).max().unwrap_or(0);
            println!(
                "|{:<max_id$}|{:<4}  |{:<max_content$}|",
                "ID",
                "Done",
                "Content",
                max_id = max_id + 1,
                max_content = max_content
            );
            let stdout = io::stdout();
            let mut out = stdout.lock();
            for todo in db.iter() {
                let done_str = if todo.is_completed() { "[X]" } else { "[ ]" };
                writeln!(
                    out,
                    "|{:<max_id$}|{:<4}  |{:<max_content$}|",
                    todo.id(),
                    done_str,
                    todo.content(),
                    max_id = max_id + 1,
                    max_content = max_content
                )
                .unwrap();
            }
            false
        }
        Commands::Rm { id } => {
            db.remove_todo(id);
            true
        }
        Commands::Done { id } => {
            let todo = db.get_todo_mut(id).expect("Todo not found");
            todo.set_completed(true);
            true
        }
        Commands::Edit { id, content } => {
            let todo = db.get_todo_mut(id).expect("Todo not found");
            todo.set_content(content.join(" "));
            true
        }
    };

    if changed {
        Storage::save(db)?;
    }

    Ok(())
}
