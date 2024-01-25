#![feature(fs_try_exists)]

pub mod commands;
pub mod structures;

use clap::{arg, command, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, author, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long, default_value = "./")]
        src_dir: PathBuf,
    },
    Serve {
        #[arg(short, long, default_value = "./")]
        src_dir: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build { src_dir }) => {
            println!("Build on src_dir: {src_dir:?}");
            commands::build(src_dir);
        }
        Some(Commands::Serve { src_dir }) => {
            println!("Serve on src_dir: {src_dir:?}");
            commands::serve(src_dir);
        }
        None => {
            println!("subcommand None")
        }
    }
}
