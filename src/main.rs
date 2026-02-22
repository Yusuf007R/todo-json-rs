use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self};
use time::OffsetDateTime;

#[derive(Subcommand, Debug)]
enum Commands {
    /// The `Add` command takes a `content` string argument
    Add { content: String },
    /// The `Rm` command takes an `id` number argument.
    Rm { id: u32 },
    /// The `Ls` command lists all todos.
    Ls,
    /// The `Done` command takes an `id` number argument.
    Done { id: u32 },
    /// The `Edit` command takes an `id` number argument and a `content` string argument.
    Edit { id: u32, content: String },
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
                content,
                completed_at: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            };
            db.todos.push(todo);
            db.next_id += 1;
            save_db(db);
        }

        Commands::List => {
            println!("{:?}", db.todos)
        }
        Commands::Remove { id } => {
            db.todos.retain(|todo| todo.id != id);
            save_db(db);
        }
    }
}
