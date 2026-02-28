use crate::model::Db;
use anyhow::{Context, Result};
use std::fs::{self};
pub struct Storage;

impl Storage {
    pub fn load() -> Result<Db> {
        let content = fs::read_to_string("todos.json")
            .context("Failed to read todos.json, has the db been initialized?")?;
        let todos: Db = serde_json::from_str(&content).context("Failed to parse todos.json")?;
        Ok(todos)
    }

    pub fn save(db: Db) -> Result<()> {
        let content =
            serde_json::to_string_pretty(&db).context("Failed to serialize db to JSON")?;
        fs::write("todos.json", content).context("Failed to write to todos.json")?;

        Ok(())
    }

    pub fn init() -> Result<()> {
        let db = Db::new();
        Self::save(db)
    }

    pub fn reset() -> Result<()> {
        fs::remove_file("todos.json").context("Failed to remove todos.json")?;
        Ok(())
    }
}
