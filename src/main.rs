use anyhow::{Context, Result};
use clap::Parser;
mod cli;
use std::io;
mod model;
mod storage;
mod ui;
use crate::cli::{Cli, TodoCommands};
use crate::storage::Storage;
use crate::ui::{JsonRenderer, Render, Renderer, TextRenderer};
use std::io::Write;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let renderer = if cli.json {
        Renderer::Json(JsonRenderer)
    } else {
        Renderer::Text(TextRenderer)
    };

    match cli.command {
        cli::Commands::Db { cmd } => match cmd {
            cli::DbCommands::Init => {
                Storage::init()?;
                writeln!(&mut out, "Database initialized").context("Failed to write to output")?;
                return Ok(());
            }
            cli::DbCommands::Reset => {
                Storage::reset()?;
                writeln!(&mut out, "Database reset").context("Failed to write to output")?;
                return Ok(());
            }
        },
        cli::Commands::Todo { cmd } => {
            let mut db = Storage::load()?;
            let changed: bool = match cmd {
                TodoCommands::Add { content } => {
                    let todo = db.add_todo(content.join(" "));
                    renderer.render_todo(&mut out, todo)?;
                    true
                }

                TodoCommands::Ls => {
                    let todos = db.todos();
                    renderer.render_todos(&mut out, todos)?;
                    false
                }
                TodoCommands::Rm { id } => {
                    let todo = db.remove_todo(id).expect("Todo not found");
                    renderer.render_todo(&mut out, &todo)?;
                    true
                }
                TodoCommands::Done { id } => {
                    let todo = db.get_todo_mut(id).expect("Todo not found");
                    let todo = todo.set_completed(true);
                    renderer.render_todo(&mut out, todo)?;
                    true
                }
                TodoCommands::Edit { id, content } => {
                    let todo = db.get_todo_mut(id).expect("Todo not found");
                    let todo = todo.set_content(content.join(" "));
                    renderer.render_todo(&mut out, todo)?;
                    true
                }
            };

            if changed {
                Storage::save(db)?;
            }
        }
    };

    Ok(())
}
