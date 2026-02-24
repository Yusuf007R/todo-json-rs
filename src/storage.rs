use crate::model::Db;
use anyhow::{Context, Result};
use std::fs::{self};
pub struct Storage;

impl Storage {
    pub fn load() -> Result<Db> {
        let content = fs::read_to_string("todos.json").context(format!(
            "Failed to read todos.json, has the db been initialized?"
        ))?;
        let todos: Db =
            serde_json::from_str(&content).context(format!("Failed to parse todos.json"))?;
        Ok(todos)
    }

    pub fn save(db: Db) -> Result<()> {
        let content =
            serde_json::to_string_pretty(&db).context(format!("Failed to serialize db to JSON"))?;
        fs::write("todos.json", content).context(format!("Failed to write to todos.json"))?;

        Ok(())
    }
}
