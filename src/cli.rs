use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum TodoCommands {
    /// The `Add` command takes a `content` string argument
    Add { content: Vec<String> },
    /// The `Rm` command takes an `id` number argument.
    Rm { id: u32 },
    /// The `Ls` command lists all todos.
    Ls,
    /// The `Done` command takes an `id` number argument.
    Done { id: u32 },
    /// The `Edit` command takes an `id` number argument and a `content` string argument.
    Edit { id: u32, content: Vec<String> },
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// The `Init` command initializes the database.
    Init,
    /// The `Reset` command resets the database.
    Reset,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// The `Todo` command is used to manage todos.
    Todo {
        #[command(subcommand)]
        cmd: TodoCommands,
    },
    /// The `Db` command is used to manage the database.
    Db {
        #[command(subcommand)]
        cmd: DbCommands,
    },
}

// 1. Group the flags to prevent them from being used together
#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct OutputFlags {
    /// Output the result as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub output_flags: OutputFlags,

    #[command(subcommand)]
    pub command: Commands,
}
