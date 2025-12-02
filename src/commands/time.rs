use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Copy)]
pub enum TimePart {
    Part1,
    Part2,
    Both,
}

impl TimePart {
    const fn flag(&self) -> &str {
        match self {
            Self::Part1 => "--part1",
            Self::Part2 => "--part2",
            Self::Both => "--timing",
        }
    }
}

pub fn time_day(day: u8, part: TimePart) -> Result<()> {
    if !(1..=25).contains(&day) {
        anyhow::bail!("Day must be between 1 and 25");
    }

    if !Path::new("Cargo.toml").exists() {
        anyhow::bail!("Not in a workspace directory. Run 'aoc-cli init <name>' first.");
    }

    let day_name = format!("day{day:02}");
    let day_path = Path::new(&day_name);

    if !day_path.exists() {
        anyhow::bail!("Day {day} does not exist. Run 'aoc-cli add {day}' first.");
    }

    println!("Running {day_name} with timing ({part:?})...\n");

    let output = Command::new("cargo")
        .args(["run", "--release", "-p", &day_name, "--", part.flag()])
        .output()
        .context(format!("Failed to run {day_name}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to run {day_name}:\n{stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{stdout}");

    Ok(())
}
