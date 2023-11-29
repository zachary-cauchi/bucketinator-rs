use crate::{app::App, model::todo::Todo};
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The name of the todo to add.
    name: String,

    /// Whether the task is completed or not.
    #[arg(value_name = "completed", default_value = "false")]
    is_completed: Option<bool>,
}

pub fn run(app: &mut App, args: Args) {
    let Args { name, is_completed } = args;

    let todo: Todo = Todo::new(name, is_completed);

    let added_todo = app.add_todo(todo).unwrap();

    println!("Adding todo '{}' to database.", added_todo.name);
}
