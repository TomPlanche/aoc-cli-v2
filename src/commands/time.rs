use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
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

fn find_workspace_root() -> Result<PathBuf> {
    let mut current_dir = env::current_dir().context("Failed to get current directory")?;

    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            // Check if it's a workspace by looking for [workspace] section
            let content =
                std::fs::read_to_string(&cargo_toml).context("Failed to read Cargo.toml")?;
            if content.contains("[workspace]") {
                return Ok(current_dir);
            }
        }

        // Try to go up one directory
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            anyhow::bail!("Not in a workspace directory. Run 'aoc-cli init <name>' first.");
        }
    }
}

fn detect_current_day() -> Result<u8> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let dir_name = current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .context("Failed to get directory name")?;

    // Check if directory name matches "dayXX" pattern
    if dir_name.starts_with("day") && dir_name.len() == 5 {
        let day_str = &dir_name[3..5];
        if let Ok(day) = day_str.parse::<u8>()
            && (1..=25).contains(&day)
        {
            return Ok(day);
        }
    }

    anyhow::bail!(
        "Could not auto-detect day number. Please provide a day number (1-25) or run from a day folder."
    )
}

pub fn time_day(day: Option<u8>, part: TimePart) -> Result<()> {
    // Determine the day number (from parameter or auto-detect)
    let day = match day {
        Some(d) => {
            if !(1..=25).contains(&d) {
                anyhow::bail!("Day must be between 1 and 25");
            }
            d
        }
        None => detect_current_day()?,
    };

    // Find the workspace root
    let workspace_root = find_workspace_root()?;

    let day_name = format!("day{day:02}");
    let day_path = workspace_root.join(&day_name);

    if !day_path.exists() {
        anyhow::bail!("Day {day} does not exist. Run 'aoc-cli add {day}' first.");
    }

    println!("Running {day_name} with timing ({part:?})...\n");

    let output = Command::new("cargo")
        .current_dir(&workspace_root)
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
