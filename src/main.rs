use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self};
use time::OffsetDateTime;

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

#[derive(Serialize, Deserialize, Debug)]
struct Db {
    next_id: u32,
    todos: Vec<Todo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    content: String,
    #[serde(with = "time::serde::rfc3339::option")]
    completed_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
}

fn load_db() -> Db {
    let content = fs::read_to_string("todos.json").unwrap_or_default();
    let todos: Db = serde_json::from_str(&content).unwrap_or(Db {
        next_id: 1,
        todos: vec![],
    });
    todos
}

fn save_db(db: Db) {
    let content = serde_json::to_string_pretty(&db).unwrap();
    fs::write("todos.json", content).unwrap();
}

fn main() {
    let cli = Cli::parse();

    let mut db = load_db();

    match cli.command {
        Commands::Add { content } => {
            let todo = Todo {
                id: db.next_id,
                content: content.join(" "),
                completed_at: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            };
            db.todos.push(todo);
            db.next_id += 1;
            save_db(db);
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
            db.todos.retain(|todo| todo.id != id);
            save_db(db);
        }
        Commands::Done { id } => {
            if let Some(todo) = db.todos.iter_mut().find(|todo| todo.id == id) {
                let now = OffsetDateTime::now_utc();
                todo.completed_at = Some(now);
                todo.updated_at = now;
                save_db(db);
            };
        }
        Commands::Edit { id, content } => {
            if let Some(todo) = db.todos.iter_mut().find(|todo| todo.id == id) {
                todo.content = content.join(" ");
                todo.updated_at = OffsetDateTime::now_utc();
                save_db(db);
            };
        }
    }
}
