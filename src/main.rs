use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Next,
    Svlete,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// set custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// set operation mode
    #[arg(short, long, value_name = "MODE")]
    mode: Option<Mode>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// initializes routeradar
    Init,

    /// add a new route
    Add,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Init) => println!("init in process"),
        Some(Commands::Add) => todo!(),
        None => todo!(),
    }
}
