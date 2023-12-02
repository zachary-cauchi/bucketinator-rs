use std::{cmp::Reverse, collections::HashMap};

use crate::{
    app::App,
    model::todo::{Id, Todo},
};
use clap::{Parser, ValueEnum};
use log::debug;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
enum CompletionFilter {
    All,
    CompleteOnly,
    IncompleteOnly,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
enum SortOrder {
    Asc,
    Desc,
}

#[derive(Parser)]
pub struct Args {
    /// Filter todos by name using a substring.
    #[arg(short, long)]
    name: Option<String>,

    /// Filter by the completion status of the todo.
    #[arg(short, long, default_value = "all")]
    completion: Option<CompletionFilter>,

    /// Sort todos by name.
    #[arg(long, default_value = "asc")]
    sort_name: Option<SortOrder>,

    /// Sort todos by completion.
    #[arg(short, long, default_value = "asc")]
    sort_completion: Option<SortOrder>,
}

fn sort_todos_to_print(
    todos: &mut Vec<&Todo>,
    sort_name: Option<SortOrder>,
    sort_completion: Option<SortOrder>,
) {
    // Sort the names first so the completed sorts will appear properly grouped. Names will still be sorted amongst each group.

    match sort_name {
        Some(SortOrder::Asc) => todos.sort_by_cached_key(|t| t.name.to_string()),
        Some(SortOrder::Desc) => todos.sort_by_cached_key(|t| Reverse(t.name.to_string())),
        None => {}
    }

    match sort_completion {
        Some(SortOrder::Asc) => todos.sort_by_key(|t| t.completed),
        Some(SortOrder::Desc) => todos.sort_by_key(|t| Reverse(t.completed)),
        None => {}
    }
}

pub fn run(app: &App, args: Args) {
    let Args {
        name,
        completion,
        sort_completion,
        sort_name,
    } = args;

    let (skip_filter_name, filter_name) = match name {
        Some(n) => (n.is_empty(), n),
        None => (true, "".to_string()),
    };
    let (skip_filter_completion, filter_completion) = match completion {
        Some(CompletionFilter::IncompleteOnly) => (false, false),
        Some(CompletionFilter::CompleteOnly) => (false, true),
        _ => (true, false),
    };

    debug!("Getting and filtering todos.");

    let todos: HashMap<&Id, &Todo> = app
        .get_todos()
        .into_iter()
        .filter(|(_, todo)| skip_filter_name || todo.name.contains(&filter_name))
        .filter(|(_, todo)| skip_filter_completion || todo.completed == filter_completion)
        .collect();

    let mut todos_to_print: Vec<&Todo> = todos.iter().map(|(_, todo)| todo).cloned().collect();

    debug!("Sorting todos.");

    sort_todos_to_print(&mut todos_to_print, sort_name, sort_completion);

    debug!("Printing {} todos.", todos_to_print.len());
    println!("{}", serde_json::to_string_pretty(&todos_to_print).unwrap());
    debug!("Printed {} todos.", todos_to_print.len());
}
