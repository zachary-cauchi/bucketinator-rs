use std::{path::{Path, PathBuf}, fs::File, io::BufReader, error::Error};
use clap::Parser;

use crate::todo::Todo;

#[derive(Parser)]
pub struct Args {
    /// Filter todos by name using a substring.
    #[arg(short, long)]
    filter: Option<String>,

    /// The database file to use for loading the todos.
    #[arg(value_parser = validate_file)]
    file: PathBuf
}

pub fn validate_file(s: &str) -> Result<PathBuf, String> {
    let path = Path::new(s);

    if path.is_file() {
        Ok(PathBuf::from(s))
    } else {
        Err(format!("Failed to validate file path '{}': Path is a not a file or doesn't exist.", s))
    }
}

/// Load the todos from the given file path.
/// Can throw an error when opening a file or when parsing the file contents as JSON.
pub fn load_todos_from_json_file(file: PathBuf) -> Result<Vec<Todo>, Box<dyn Error>> {
    let db_file = File::open(file)?;
    let db_file_buffer = BufReader::new(db_file);

    let todos = serde_json::from_reader(db_file_buffer)?;

    Ok(todos)
}

pub fn filter_by_name_substring(todos: Vec<Todo>, filter: Option<String>) -> Vec<Todo> {
    match filter {
        Some(filter) => {
            todos
                .into_iter()
                .filter(|todo| todo.name.contains(&filter))
                .collect()
        },
        None => todos
    }
}

pub fn run(args: Args) {
    let Args {
        filter,
        file
    } = args;

    println!("Loading todos from file {}", file.display());

    let todos: Vec<Todo> = load_todos_from_json_file(file)
        .unwrap_or_else(|msg| panic!("Failed to load db file: {}", msg));

    println!("Loaded {} todos.", todos.len());

    let todos = filter_by_name_substring(todos, filter);

    println!("Printing {} todos.", todos.len());

    println!("{}", serde_json::to_string_pretty(&todos).unwrap());
}
