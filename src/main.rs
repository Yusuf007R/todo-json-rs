use anyhow::Result;
use clap::Parser;
mod cli;
use std::io;
mod model;
mod storage;
mod ui;
use crate::cli::{Cli, Commands};
use crate::storage::Storage;
use crate::ui::{JsonRenderer, Render, Renderer, TextRenderer};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut db = Storage::load()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let renderer = if cli.json {
        Renderer::Json(JsonRenderer)
    } else {
        Renderer::Text(TextRenderer)
    };

    let changed: bool = match cli.command {
        Commands::Add { content } => {
            let todo = db.add_todo(content.join(" "));
            renderer.render_todo(&mut out, todo)?;
            true
        }

        Commands::Ls => {
            let todos = db.todos();
            renderer.render_todos(&mut out, todos)?;
            false
        }
        Commands::Rm { id } => {
            let todo = db.remove_todo(id).expect("Todo not found");
            renderer.render_todo(&mut out, &todo)?;
            true
        }
        Commands::Done { id } => {
            let todo = db.get_todo_mut(id).expect("Todo not found");
            let todo = todo.set_completed(true);
            renderer.render_todo(&mut out, todo)?;
            true
        }
        Commands::Edit { id, content } => {
            let todo = db.get_todo_mut(id).expect("Todo not found");
            let todo = todo.set_content(content.join(" "));
            renderer.render_todo(&mut out, todo)?;
            true
        }
    };

    if changed {
        Storage::save(db)?;
    }

    Ok(())
}
