
use std::fs::{File, self};
use clap::{Args, Parser, Subcommand};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use crate::todos::Todos;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  New(NewArgs),
  Use(UseArgs),
  List(ListArgs),
  Add(AddArgs),
  Del(DelArgs),
  Ch(ChArgs),
}

/// Usage: rtd new <TODOLIST>
#[derive(Args)]
struct NewArgs {
  todolist: String,
}

/// Usage: rtd use <TODOLIST>
#[derive(Args)]
struct UseArgs {
  todolist: String,
}

/// Usage: rtd list 
#[derive(Args)]
struct ListArgs {
  #[arg(short, long)]
  table: bool,
}

/// Usage: rtd add <EVENT>
#[derive(Args)]
struct AddArgs {
  event: String,
}

/// Usage: rtd del <ID>
#[derive(Args)]
struct DelArgs {
  id: u32,
  #[arg(short, long)]
  table: bool,
}

#[derive(Args)]
struct ChArgs {
  id: u32,
  #[arg(short, long)]
  reset: bool,
}

impl Cli {
  pub fn new() -> Cli {
    Cli::parse()
  }

  pub fn run(&self) -> Result<()> {
    let mut todos = Todos::new();
    let mut isload = false;
    let mut file_path = String::from("");
    match self.find_now() {
      Ok(file) => {
        file_path = format!("{}{}", "./cache/", file);
        if let Ok(loaded_todos) = Todos::load(&file_path) {
          todos = loaded_todos;
          isload = true;
        } else {

        }
      },
      Err(_) => {
        // println!("Can not find todos");
      }
    }
    match &self.command {
      Commands::New(new_args) => {
        // println!("calling new command!");
        file_path = self.create_new(&new_args.todolist);
        todos = Todos::new();
        todos.save(file_path);
      },
      Commands::Use(use_args) => {
        // println!("calling use command!");
        self.change_now(&use_args.todolist);
      },
      Commands::List(list_args) => {
        // println!("calling list command!");
        if list_args.table {
          println!("Your todo table :");
          println!("------------------------");
          self.list();
        } else {
          if isload {
            println!("Your todos (at {}) :", file_path);
            println!("------------------------");
            todos.list();
            todos.save(file_path);
          } else {
            println!("No todos selected!");
          }
        }
      },
      Commands::Add(add_args) => {
        // println!("calling add command!");
        if isload {
          todos.add(&add_args.event);
          todos.save(file_path);
        } else {
          println!("No todos selected!");
        }
      },
      Commands::Del(del_args) => {
        // println!("calling del command!");
        if del_args.table {
          self.del(del_args.id as usize);
        } else {
          if isload {
            todos.del(del_args.id as usize);
            todos.save(file_path);
          } else {
            println!("No todos selected!");
          }
        }
      },
      Commands::Ch(ch_args) => {
        if isload {
          todos.change(ch_args.id as usize, !ch_args.reset);
          todos.save(file_path);
        } else {
          println!("No todos selected!");
        }
      }
      _ => {

      }
    }
    Ok(())
  }

  pub fn del(&self, id: usize) {
    let todos_table = Cli::find_todos_table();
    if id < todos_table.len() {
      let file_path = format!("{}{}", "./cache/", todos_table[id]);
      match fs::remove_file(file_path) {
        Ok(_) => {
          println!("Deleted successfully!");
        },
        Err(e) => {
          println!("Error deleting: {}", e);
        }
      }
    } else {
      println!("Index out of range");
    }
  }

  pub fn create_new(&self, todos: &String) -> String {
    let mut file = todos.clone();
    let mut file_path = String::from("");
    file.push_str(".json");
    if Cli::find_todos_table().contains(&file) {
      println!("{} is already exist!", todos);
    } else {
      file_path = format!("{}{}", "./cache/", file);
      match File::create(&file_path) {
        Ok(_) => {
          println!("{} created successfully.", todos);
        },
        Err(err) => {
          eprintln!("Error creating todos: {}", err);
        },
      }
    }
    file_path
  }

  pub fn list(&self) {
    let todos_table = Cli::find_todos_table();
    for (id, todos) in todos_table.iter().enumerate() {
      let todos = &todos[..todos.len() - 5];
      println!("id: {} \t {}", id, todos);
    }
  }

  pub fn find_todos_table() -> Vec<String> {
    let mut res = vec![];
    match fs::create_dir("./cache") {
      Ok(_) => {

      },
      Err(_) => {

      }
    }
    if let Ok(files) = fs::read_dir("./cache") {
      for file in files {
        if let Ok(file) = file {
          let file_name = file.file_name();
          if let Some(file_name) = file_name.to_str() {
            if let Some(first_char) = file_name.chars().nth(0) {
              if first_char != '_' {
                res.push(String::from(file_name));
              }
            } else {
              println!("The filename is empty.");
            }
          } else {
            println!("Invalid file name");
          }
        } else {
          println!("Error reading directory entry");
        }
      }
    } else {
      println!("Error reading directory");
    }
    res
  }

  pub fn find_now(&self) -> Result<String, &'static str> {
    if let Ok(file) = fs::read_to_string("./cache/_settings.json") {
      match serde_json::from_str::<String>(&file) {
        Ok(file_name) => {
          Ok(file_name)
        },
        Err(e) => {
          Err("failed to load cache")
        }
      }
    } else {
      Err("failed to read file")
    }
  }

  pub fn change_now(&self, todos: &String) -> Result<()> {
    let mut file = todos.clone();
    file.push_str(".json");
    if Cli::find_todos_table().contains(&file) {
      let serialized = serde_json::to_string(&file).with_context(|| "Unable to serialize to JSON")?;
      fs::write("./cache/_settings.json", serialized).with_context(|| "Unable to write to file")?;
      println!("Select todo table {}.", todos);
    } else {
      println!("{} is not exist!", todos);
    }
    Ok(())
  }
}