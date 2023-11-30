use std::collections::HashMap;

use crate::{
    app::App,
    model::todo::{Id, Todo},
};
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Filter todos by name using a substring.
    #[arg(short, long)]
    filter: Option<String>,
}

pub fn filter_by_name_substring(
    todos: &HashMap<Id, Todo>,
    filter: Option<String>,
) -> HashMap<Id, Todo> {
    // TODO: Find a better way of filtering without cloning.
    match filter {
        Some(filter) => todos
            .iter()
            .filter(|(_, todo)| todo.name.contains(&filter))
            .map(|(id, entry)| (*id, entry.clone()))
            .collect(),
        None => todos.clone(),
    }
}

pub fn run(app: &App, args: Args) {
    let Args { filter } = args;

    let todos = app.get_todos();
    let todos_filtered = filter_by_name_substring(&todos, filter);
    let todos_to_print: Vec<&Todo> = todos_filtered.values().collect();

    println!("Printing {} todos.", todos_to_print.len());

    println!("{}", serde_json::to_string_pretty(&todos_to_print).unwrap());
}
