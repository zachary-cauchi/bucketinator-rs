use crate::{app::App, model::todo::Id};
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The id of the todo to delete.
    id: Id,
}

pub fn run(app: &mut App, args: Args) {
    let Args { id } = args;

    println!("Removing todo with id {} to database.", id);

    let removed_todo = app.rm_todo(id);

    match removed_todo {
        Some(t) => println!("Removed todo '{}' from database.", t.name),
        None => println!("Could not find todo with id {} to remove.", id),
    }
}
