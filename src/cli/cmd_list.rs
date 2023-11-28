use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Filter todos by name using a substring.
    #[arg(long)]
    filter: Option<String>,

    /// The database file to use for loading the todos.
    file: String
}

pub fn run(args: Args) {
    let Args {
        filter,
        file
    } = args;

    println!("Loading todos from file {}", file);
}
