use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod templates;

#[derive(Parser)]
#[command(name = "aoc-cli")]
#[command(about = "A CLI tool for managing Advent of Code projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize a new AOC project")]
    Init {
        #[arg(help = "Project name (e.g., aoc2025)")]
        name: String,
    },
    #[command(about = "Add a new day to the project")]
    Add {
        #[arg(help = "Day number (1-25)")]
        day: u8,
    },
    #[command(about = "Run analytics on all day crates and output timing information")]
    Analytics {
        #[arg(help = "Output file path", default_value = "analytics.txt")]
        file_path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => commands::init_project(&name),
        Commands::Add { day } => commands::add_day(day),
        Commands::Analytics { file_path } => commands::run_analytics(&file_path),
    }
}
