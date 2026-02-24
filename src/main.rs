use clap::{Parser, Subcommand};
use std::fs::{self};
use time::OffsetDateTime;

mod model;
use crate::model::{Db, Todo};

use std::io::{self, Write};

#[derive(Subcommand, Debug)]
enum Commands {
    /// The `Add` command takes a `content` string argument
    Add { content: Vec<String> },
    /// The `Rm` command takes an `id` number argument.
    Rm { id: u32 },
    /// The `Ls` command lists all todos.
    Ls,
    /// The `Done` command takes an `id` number argument.
    Done { id: u32 },
    /// The `Edit` command takes an `id` number argument and a `content` string argument.
    Edit { id: u32, content: Vec<String> },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn load_db() -> Db {
    let content = fs::read_to_string("todos.json").unwrap_or_default();
    let todos: Db = serde_json::from_str(&content).unwrap_or(Db::new());
    todos
}

fn save_db(db: Db) {
    let content = serde_json::to_string_pretty(&db).unwrap();
    fs::write("todos.json", content).unwrap();
}

fn main() {
    let cli = Cli::parse();

    let mut db = load_db();

    let changed: bool = match cli.command {
        Commands::Add { content } => {
            db.add_todo(content.join(" "));
            true
        }

        Commands::Ls => {
            let max_id = db
                .todos
                .iter()
                .map(|t| t.id.to_string().len())
                .max()
                .unwrap_or(0);
            let max_content = db.todos.iter().map(|t| t.content.len()).max().unwrap_or(0);
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
            for todo in db.todos.iter() {
                let done_str = if todo.completed_at.is_some() {
                    format!("[x]")
                } else {
                    "[ ]".to_string()
                };
                writeln!(
                    out,
                    "|{:<max_id$}|{:<4}  |{:<max_content$}|",
                    todo.id,
                    done_str,
                    todo.content,
                    max_id = max_id + 1,
                    max_content = max_content
                )
                .unwrap();
            }
        }
        Commands::Rm { id } => {
            db.remove_todo(id);
            true
        }
        Commands::Done { id } => {
            let todo = db.get_todo_mut(id);
            todo.set_completed(true);
            true
        }
        Commands::Edit { id, content } => {
            let todo = db.get_todo_mut(id);
            todo.set_content(content.join(" "));
            true
        }
    };

    if changed {
        save_db(db);
    }
}
