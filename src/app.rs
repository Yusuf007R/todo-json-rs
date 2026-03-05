use crate::cli::{Commands, DbCommands, OutputFlags, TodoCommands};
use crate::storage::Storage;
use crate::ui::{Render, Renderer};
use anyhow::{Context, Result};
use std::io::Write;
pub struct App {
    renderer: Renderer,
}

impl App {
    pub fn new(flag: OutputFlags) -> Self {
        App {
            renderer: Renderer::new(flag),
        }
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
                Storage::init()?;
                writeln!(out, "Database initialized").context("Failed to write to output")?;
                Ok(())
            }
            DbCommands::Reset => {
                Storage::reset()?;
                writeln!(out, "Database reset").context("Failed to write to output")?;
                Ok(())
            }
        }
    }

    fn handle_todo_command(&self, cmd: TodoCommands, out: &mut impl Write) -> Result<()> {
        let mut db = Storage::load()?;
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
                let todo = todo.set_completed(true);
                self.renderer.render_todo(out, todo)?;
                true
            }
            TodoCommands::Edit { id, content } => {
                let todo = db.get_todo_mut(id).context("Todo not found")?;
                let todo = todo.set_content(content.join(" "));
                self.renderer.render_todo(out, todo)?;
                true
            }
        };

        if changed {
            Storage::save(db)?;
        }
        Ok(())
    }
}
