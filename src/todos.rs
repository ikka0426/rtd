
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize)]
struct Todo {
  event: String,
  completed: bool
}

#[derive(Serialize, Deserialize)]
pub struct Todos {
  todo_list: Vec<Todo>,
}

impl Todos {
  pub fn new() -> Todos {
    Todos {
      todo_list: vec![]
    }
  }

  pub fn list(&self) {
    for (index, todo) in self.todo_list.iter().enumerate() {
      println!("id: {} \t {}  {}", index, if todo.completed { "✅" } else { "⬜" }, todo.event);
    }
  }

  pub fn add(&mut self, event: &String) {
    self.todo_list.push(Todo {
      event: event.clone(),
      completed: false
    });
  }

  pub fn del(&mut self, id: usize) {
    if id < self.todo_list.len() {
      self.todo_list.remove(id);
      println!("Deleted successfully!");
    } else {
      println!("Index out of range");
    }
  }

  pub fn change(&mut self, id: usize, status: bool) {
    if id < self.todo_list.len() {
      self.todo_list[id].completed = status;
      println!("Change status successfully!");
    } else {
      println!("Index out of range");
    }
  }

  pub fn save(&self, file: String) -> Result<()> {
    let serialized = serde_json::to_string(self).with_context(|| "Unable to serialize to JSON")?;
    fs::write(file, serialized).with_context(|| "Unable to write to file")?;
    Ok(())
  }

  pub fn load(file: &String) -> Result<Self, &'static str> {
    if let Ok(file) = fs::read_to_string(file) {
      match serde_json::from_str::<Todos>(&file) {
        Ok(todos_json) => {
          let mut todos = Todos::new();
          for todo in todos_json.todo_list {
            todos.todo_list.push(Todo {
              event: todo.event,
              completed: todo.completed
            });
          }
          Ok(todos)
        },
        Err(e) => {
          println!("e");
          Err("failed to load cache")
        }
      }
    } else {
      Err("failed to read file")
    }
  }
}
