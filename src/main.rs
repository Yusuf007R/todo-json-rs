use anyhow::Result;
use clap::Parser;
mod cli;
use std::io;
mod app;
mod model;
mod storage;
mod ui;
use crate::app::App;
use crate::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let app = App::new(cli.output_flags);

    app.run(cli.command, &mut out)
}
