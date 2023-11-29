use clap::{Parser, Subcommand};

mod cmd_add;
mod cmd_list;

#[derive(Parser)]
#[command(author = "Zachary Cauchi")]
#[command(version = "0.1.0")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List all the todos.
    List(cmd_list::Args),

    /// Add a new todo.
    Add(cmd_add::Args),
}

/// Main entrypoint for the cli interface.
pub fn enter_cli() {
    let cli = Cli::parse();

    println!("Hello, world!");

    let immediate_result = match cli.command {
        Command::List(args) => cmd_list::run(args),

        Command::Add(args) => cmd_add::run(args),
    };

    return immediate_result;
}
