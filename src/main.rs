#![feature(fs_try_exists)]

pub mod commands;
pub mod structures;

use std::path::PathBuf;
use clap::{Parser, Subcommand, arg, Command, command, value_parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long, default_value = "./")]
        src_dir: PathBuf,
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Build { src_dir}) => {
            // println!("{src_dir:?}");
            commands::build::build(src_dir);
        }
        None => {
            // println!("subcommand None")
        }
    }
}
