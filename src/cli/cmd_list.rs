use crate::model::{db::TodoLoader, todo::Todo};
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
pub struct Args {
    /// Filter todos by name using a substring.
    #[arg(short, long)]
    filter: Option<String>,

    /// The database file to use for loading the todos.
    #[arg(value_parser = validate_file)]
    file: PathBuf,
}

pub fn validate_file(s: &str) -> Result<PathBuf, String> {
    let path = Path::new(s);

    if path.is_file() {
        Ok(PathBuf::from(s))
    } else {
        Err(format!(
            "Failed to validate file path '{}': Path is a not a file or doesn't exist.",
            s
        ))
    }
}

pub fn filter_by_name_substring(todos: Vec<Todo>, filter: Option<String>) -> Vec<Todo> {
    match filter {
        Some(filter) => todos
            .into_iter()
            .filter(|todo| todo.name.contains(&filter))
            .collect(),
        None => todos,
    }
}

pub fn run(args: Args) {
    let Args { filter, file } = args;

    println!("Loading todos from file {}", file.display());

    let todos: Vec<Todo> = TodoLoader::load_todos(file);

    println!("Loaded {} todos.", todos.len());

    let todos = filter_by_name_substring(todos, filter);

    println!("Printing {} todos.", todos.len());

    println!("{}", serde_json::to_string_pretty(&todos).unwrap());
}
