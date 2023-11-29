use crate::{app::App, model::todo::Todo};
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Filter todos by name using a substring.
    #[arg(short, long)]
    filter: Option<String>,
}

pub fn filter_by_name_substring(todos: &Vec<Todo>, filter: Option<String>) -> Vec<Todo> {
    match filter {
        Some(filter) => todos
            .iter()
            .filter(|todo| todo.name.contains(&filter))
            .cloned()
            .collect(),
        None => todos.clone(),
    }
}

pub fn run(app: &App, args: Args) {
    let Args { filter } = args;

    let todos = app.get_todos();
    let todos_filtered = filter_by_name_substring(&todos, filter);

    println!("Printing {} todos.", todos_filtered.len());

    println!("{}", serde_json::to_string_pretty(&todos_filtered).unwrap());
}
