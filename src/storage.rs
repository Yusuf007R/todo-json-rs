use crate::model::Db;
use anyhow::{Context, Result};
use dirs::data_dir;
use std::{
    fs::{self},
    path::PathBuf,
};
pub struct Storage {
    db_file_path: PathBuf,
}

impl Storage {
    pub fn new(db_dir: Option<PathBuf>) -> Result<Self> {
        let db_dir = match db_dir {
            Some(dir) => dir,
            None => Self::default_db_dir()?,
        };

        if !db_dir.exists() {
            fs::create_dir_all(&db_dir).context("Failed to create database directory")?;
        }

        Ok(Storage {
            db_file_path: db_dir.join("todos.json"),
        })
    }

    pub fn default_db_dir() -> Result<PathBuf> {
        let path =
            data_dir().context("Could not determine data directory for the current platform")?;

        Ok(path.join("todo-json-rs"))
    }

    pub fn load(&self) -> Result<Db> {
        let content = fs::read_to_string(&self.db_file_path).with_context(|| {
            format!(
                "Failed to read `{}`, has the db been initialized?, try using command `db init`",
                self.db_file_path.display()
            )
        })?;
        let todos: Db = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse `{}`", self.db_file_path.display()))?;
        Ok(todos)
    }

    pub fn save(&self, db: &Db) -> Result<()> {
        let content =
            serde_json::to_string_pretty(&db).context("Failed to serialize db to JSON")?;
        fs::write(&self.db_file_path, content)
            .with_context(|| format!("Failed to write to `{}`", self.db_file_path.display()))?;

        Ok(())
    }

    pub fn init(&self) -> Result<()> {
        let db = Db::new();
        self.save(&db)
    }

    pub fn reset(&self) -> Result<()> {
        fs::remove_file(&self.db_file_path)
            .with_context(|| format!("Failed to remove `{}`", self.db_file_path.display()))?;
        Ok(())
    }
}
