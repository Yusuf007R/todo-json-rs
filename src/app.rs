use crate::cli::{Commands, DbCommands, OutputFlags, TodoCommands};
use crate::storage::Storage;
use crate::ui::{Render, Renderer};
use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;

pub struct App {
    renderer: Renderer,
    storage: Storage,
}

impl App {
    pub fn new(flag: OutputFlags, db_path: Option<PathBuf>) -> Result<Self> {
        Ok(App {
            renderer: Renderer::new(flag),
            storage: Storage::new(db_path)?,
        })
    }

    pub fn run(&self, cmd: Commands, out: &mut impl Write) -> Result<()> {
        match cmd {
            Commands::Db { cmd } => self.handle_db_command(cmd, out),
            Commands::Todo { cmd } => self.handle_todo_command(cmd, out),
        }
    }

    fn handle_db_command(&self, cmd: DbCommands, out: &mut impl Write) -> Result<()> {
        match cmd {
            DbCommands::Init => {
                self.storage.init()?;
                writeln!(out, "Database initialized").context("Failed to write to output")?;
                Ok(())
            }
            DbCommands::Reset => {
                self.storage.reset()?;
                writeln!(out, "Database reset").context("Failed to write to output")?;
                Ok(())
            }
        }
    }

    fn handle_todo_command(&self, cmd: TodoCommands, out: &mut impl Write) -> Result<()> {
        let mut db = self.storage.load()?;
        let changed = match cmd {
            TodoCommands::Add { content } => {
                let todo = db.add_todo(content.join(" "));
                self.renderer.render_todo(out, todo)?;
                true
            }

            TodoCommands::Ls => {
                let todos = db.todos();
                self.renderer.render_todos(out, todos)?;

                false
            }
            TodoCommands::Rm { id } => {
                let todo = db.remove_todo(id).context("Todo not found")?;
                self.renderer.render_todo(out, &todo)?;
                true
            }
            TodoCommands::Done { id } => {
                let todo = db.get_todo_mut(id).context("Todo not found")?;
                todo.set_completed(true);
                self.renderer.render_todo(out, todo)?;
                true
            }
            TodoCommands::Edit { id, content } => {
                let todo = db.get_todo_mut(id).context("Todo not found")?;
                todo.set_content(content.join(" "));
                self.renderer.render_todo(out, todo)?;
                true
            }
        };

        if changed {
            self.storage.save(&db)?;
        }
        Ok(())
    }
}
