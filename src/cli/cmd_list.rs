use std::collections::HashMap;

use crate::{
    app::App,
    model::todo::{Id, Todo},
};
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
enum CompletionFilter {
    All,
    CompleteOnly,
    IncompleteOnly,
}

#[derive(Parser)]
pub struct Args {
    /// Filter todos by name using a substring.
    #[arg(short, long)]
    name: Option<String>,

    /// Filter by the completion status of the todo.
    #[arg(short, long, default_value = "all")]
    completion: Option<CompletionFilter>,
}

pub fn run(app: &App, args: Args) {
    let Args { name, completion } = args;
    let (skip_filter_name, filter_name) = match name {
        Some(n) => (n.is_empty(), n),
        None => (true, "".to_string()),
    };
    let (skip_filter_completion, filter_completion) = match completion {
        Some(CompletionFilter::IncompleteOnly) => (false, false),
        Some(CompletionFilter::CompleteOnly) => (false, true),
        _ => (true, false),
    };

    let todos: HashMap<&Id, &Todo> = app
        .get_todos()
        .into_iter()
        .filter(|(_, todo)| skip_filter_name || todo.name.contains(&filter_name))
        .filter(|(_, todo)| skip_filter_completion || todo.completed == filter_completion)
        .collect();

    let todos_to_print: Vec<&Todo> = todos.iter().map(|(_, todo)| todo).cloned().collect();

    println!("Printing {} todos.", todos_to_print.len());

    println!("{}", serde_json::to_string_pretty(&todos_to_print).unwrap());
}
