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
        #[arg(help = "Output file path", default_value = "analytics.md")]
        file_path: String,
    },
    #[command(about = "Time a specific day's solution")]
    Time {
        #[arg(help = "Day number (1-25). Auto-detected if in a day folder.")]
        day: Option<u8>,
        #[arg(long, help = "Time only part 1")]
        part1: bool,
        #[arg(long, help = "Time only part 2")]
        part2: bool,
    },
    #[command(about = "Update the utils crate to the latest version")]
    Update {
        #[arg(help = "Component to update (currently only 'utils')")]
        component: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => commands::init_project(&name),
        Commands::Add { day } => commands::add_day(day),
        Commands::Analytics { file_path } => commands::run_analytics(&file_path),
        Commands::Time { day, part1, part2 } => {
            let part = match (part1, part2) {
                (true, true) => anyhow::bail!("Cannot specify both --part1 and --part2"),
                (true, false) => commands::TimePart::Part1,
                (false, true) => commands::TimePart::Part2,
                (false, false) => commands::TimePart::Both,
            };
            commands::time_day(day, part)
        }
        Commands::Update { component } => {
            if component.to_lowercase() == "utils" {
                commands::update_utils()
            } else {
                anyhow::bail!(
                    "Unknown component '{component}'. Currently only 'utils' is supported."
                )
            }
        }
    }
}
