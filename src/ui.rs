use std::io::StdoutLock;
use std::io::{self, Write};
use crate::model::Todo;
use anyhow::{Result, Context};

trait Render{

    fn render_todo(&self, out:&StdoutLock, todo:Todo) -> Result<()>;

    fn render_todos(&self, out:&StdoutLock, todos: Vec<Todo>) -> Result<()>
}



pub struct TextRenderer;
pub struct JsonRenderer;

impl Render for JsonRenderer{

    fn render_todo(&self, out:&nStdoutLock,  todo:Todo) -> Result<()>{
        let json = serde_json::to_string(&todo).context("Failed to serialize todo to JSON")?;
        writeln!(out, "{}", json).context("Failed to write JSON to output")?;
        Ok(())
    }

    fn render_todos(&self, out:&StdoutLock, todos: Vec<Todo>) -> Result<()>{
        let json = serde_json::to_string(&todos).context("Failed to serialize todos to JSON")?;
writeln!(out, "{}", json).context("Failed to write JSON to output")?;
        Ok(())
    }
}






