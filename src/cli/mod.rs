use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    List {
        /// Filter by a substring pattern.
        pattern: Option<String>,
    },

    Add {
        /// The name of the todo to add.
        name: String,

        /// Whether the task is completed or not.
        #[arg(value_name = "compleed", default_value = "false")]
        is_completed: Option<bool>,
    }
}

/// Main entrypoint for the cli interface.
pub fn enter_cli() {
    let cli = Cli::parse();

    println!("Hello, world!");

    let immediate_result = match &cli.command {
        Some(Commands::List { pattern }) => {
            println!("Listing current todos.");
        },

        Some(Commands::Add { name, is_completed }) => {
            println!("Adding new todo \"{}\".", name);
        },

        None => {
            println!("No command to run.");
        },
    };

    return immediate_result;
}
