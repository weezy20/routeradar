#![deny(missing_docs)]
//! routeradar is a cli tool intended to help with file based routing for nextjs and sveltejs

use std::{io::Write, path::PathBuf};

use clap::{Parser, Subcommand};
use routeradar::config;
use std::fs::File;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None, arg_required_else_help=true)]
struct Args {
    /// set custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// set operation mode
    #[arg(short, long, value_name = "MODE")]
    mode: Option<config::Mode>,

    /// set path manually
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

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

    /// temp command for debug
    Deb,
}

fn main() {
    // let schema = schemars::schema_for!(config::RR);
    // let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    //
    // let mut file = File::create("routeradar_schema.json").unwrap();
    // file.write_all(schema_json.as_bytes()).unwrap();
    let args = Args::parse();

    match &args.command {
        Commands::Init => todo!(),
        Commands::Add => todo!(),
        Commands::Show => todo!(),
        Commands::Gen => todo!(),
        Commands::Deb => {
            let mode = routeradar::scanner::get_mode(&args.path.unwrap());
            match mode {
                Ok(mode) => {
                    println!("{:?}", mode);
                }
                Err(error) => {
                    println!("{}", error)
                }
            }
            // println!("{:?}", mode);
        }
    }
}
