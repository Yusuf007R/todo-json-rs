use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Db {
    next_id: u32,
    todos: Vec<Todo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    content: String,
    #[serde(with = "time::serde::rfc3339::option")]
    completed_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
}

impl Todo {
    pub fn new(id: u32, content: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id,
            content,
            completed_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn set_completed(&mut self, completed: bool) {
        if completed {
            self.completed_at = Some(OffsetDateTime::now_utc());
        } else {
            self.completed_at = None;
        }
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn completed_at(&self) -> Option<OffsetDateTime> {
        self.completed_at
    }

    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }

    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }
}

impl Db {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            todos: Vec::new(),
        }
    }

    pub fn todos(&self) -> &[Todo] {
        &self.todos
    }

    pub fn add_todo(&mut self, content: String) -> &Todo {
        let todo = Todo::new(self.next_id, content);
        self.todos.push(todo);
        self.next_id += 1;
        self.todos.last().unwrap()
    }

    pub fn remove_todo(&mut self, id: u32) -> Option<Todo> {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            Some(self.todos.remove(pos))
        } else {
            None
        }
    }

    pub fn get_todo(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|t| t.id == id)
    }

    pub fn get_todo_mut(&mut self, id: u32) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|t| t.id == id)
    }

    pub fn get_next_id(&self) -> u32 {
        self.next_id
    }
}
