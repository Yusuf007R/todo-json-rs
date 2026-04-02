use crate::{cli, model::Todo};
use anyhow::{Context, Result};
use std::io::Write;

pub trait Render {
    fn render_todo(&self, out: &mut impl Write, todo: &Todo) -> Result<()>;
    fn render_todos(&self, out: &mut impl Write, todos: &[&Todo]) -> Result<()>;
}

pub enum Renderer {
    Json(JsonRenderer),
    Text(TextRenderer),
}

impl Renderer {
    pub fn new(flag: cli::OutputFlags) -> Self {
        if flag.json {
            Renderer::Json(JsonRenderer)
        } else {
            Renderer::Text(TextRenderer)
        }
    }
}

pub struct TextRenderer;
pub struct JsonRenderer;

impl Render for JsonRenderer {
    fn render_todo(&self, out: &mut impl Write, todo: &Todo) -> Result<()> {
        let json = serde_json::to_string(&todo).context("Failed to serialize todo to JSON")?;
        writeln!(out, "{}", json).context("Failed to write JSON to output")?;
        Ok(())
    }

    fn render_todos(&self, out: &mut impl Write, todos: &[&Todo]) -> Result<()> {
        let json = serde_json::to_string(&todos).context("Failed to serialize todos to JSON")?;
        writeln!(out, "{}", json).context("Failed to write JSON to output")?;
        Ok(())
    }
}

struct TextRendererLayout {
    max_id_len: usize,
    max_content_len: usize,
}

impl TextRenderer {
    fn internal_render_todo(
        &self,
        out: &mut impl Write,
        todo: &Todo,
        layout: &TextRendererLayout,
    ) -> Result<()> {
        let done = if todo.is_completed() { "[x]" } else { "[ ]" };
        writeln!(
            out,
            "|{:<max_id$}|{:<4}  |{:<max_content$}|",
            todo.id(),
            done,
            todo.content(),
            max_id = layout.max_id_len,
            max_content = layout.max_content_len
        )
        .context("Failed to write todo to output")?;
        Ok(())
    }

    fn compute_widths(&self, todos: &[&Todo]) -> TextRendererLayout {
        let max_id = todos
            .iter()
            .map(|t| t.id().to_string().len())
            .max()
            .unwrap_or(0);
        let max_content = todos.iter().map(|t| t.content().len()).max().unwrap_or(0);
        TextRendererLayout {
            max_id_len: max_id + 1,
            max_content_len: max_content,
        }
    }

    fn print_header(&self, out: &mut impl Write, layout: &TextRendererLayout) -> Result<()> {
        writeln!(
            out,
            "|{:<max_id$}|{:<4}  |{:<max_content$}|",
            "ID",
            "Done",
            "Content",
            max_id = layout.max_id_len,
            max_content = layout.max_content_len
        )
        .context("Failed to write header to output")?;
        Ok(())
    }
}

impl Render for TextRenderer {
    fn render_todo(&self, out: &mut impl Write, todo: &Todo) -> Result<()> {
        let layout = self.compute_widths(std::slice::from_ref(&todo));
        self.print_header(out, &layout)?;
        self.internal_render_todo(out, todo, &layout)
    }

    fn render_todos(&self, out: &mut impl Write, todos: &[&Todo]) -> Result<()> {
        let layout = self.compute_widths(todos);
        self.print_header(out, &layout)?;
        for todo in todos {
            self.internal_render_todo(out, todo, &layout)?;
        }
        Ok(())
    }
}

impl Render for Renderer {
    fn render_todo(&self, out: &mut impl Write, todo: &Todo) -> Result<()> {
        match self {
            Renderer::Json(r) => r.render_todo(out, todo),
            Renderer::Text(r) => r.render_todo(out, todo),
        }
    }
    fn render_todos(&self, out: &mut impl Write, todos: &[&Todo]) -> Result<()> {
        match self {
            Renderer::Json(r) => r.render_todos(out, todos),
            Renderer::Text(r) => r.render_todos(out, todos),
        }
    }
}
