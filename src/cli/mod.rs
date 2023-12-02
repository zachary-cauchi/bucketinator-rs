use clap::{Parser, Subcommand};

use crate::app::App;

mod cmd_add;
mod cmd_list;
mod cmd_rm;

#[derive(Parser)]
#[command(author = "Zachary Cauchi")]
#[command(version = "0.1.0")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List all the todos.
    List(cmd_list::Args),

    /// Add a new todo.
    Add(cmd_add::Args),

    /// Remove a todo.
    Rm(cmd_rm::Args),
}

/// Main entrypoint for the cli interface.
pub fn enter_cli(app: &mut App) {
    let cli = Cli::parse();

    // Initialise the app after parsing the cli (where the cli may exit early such as when printing help info).
    app.initialize();

    match cli.command {
        Command::List(args) => cmd_list::run(app, args),

        Command::Add(args) => cmd_add::run(app, args),

        Command::Rm(args) => cmd_rm::run(app, args),
    };
}
