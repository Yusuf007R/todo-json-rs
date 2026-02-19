use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    /// The `Add` command takes a `todo` argument, which is a string.
    Add { todo: String },
    /// The `Remove` command takes an `id` argument, which is a number.
    Remove { id: u32 },
    /// The `List` command lists all todos.
    List,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { todo } => {
            println!("adding {}", todo)
        }
        Commands::List => {
            println!("listing")
        }
        Commands::Remove { id } => {
            println!("removing {}", id)
        }
    }
}
