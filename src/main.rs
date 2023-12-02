use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Next,
    Svlete,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None, arg_required_else_help=true)]
struct Args {
    /// set custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// set operation mode
    #[arg(short, long, value_name = "MODE")]
    mode: Option<Mode>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// initializes routeradar
    Init,

    /// add a new route
    Add,

    /// displays all the routes in the project
    Show,

    /// generates routes from a config file
    Gen,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Init => todo!(),
        Commands::Add => todo!(),
        Commands::Show => todo!(),
        Commands::Gen => todo!(),
    }
}
