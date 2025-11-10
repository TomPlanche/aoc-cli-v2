use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Clone)]
struct DayTiming {
    day: u8,
    part1_micros: u128,
    part2_micros: u128,
}

impl DayTiming {
    const fn total_micros(&self) -> u128 {
        self.part1_micros + self.part2_micros
    }

    #[allow(clippy::cast_precision_loss)]
    fn part1_ms(&self) -> f64 {
        self.part1_micros as f64 / 1000.0
    }

    #[allow(clippy::cast_precision_loss)]
    fn part2_ms(&self) -> f64 {
        self.part2_micros as f64 / 1000.0
    }

    #[allow(clippy::cast_precision_loss)]
    fn total_ms(&self) -> f64 {
        self.total_micros() as f64 / 1000.0
    }
}

pub fn run_analytics(file_path: &str) -> Result<()> {
    if !Path::new("Cargo.toml").exists() {
        anyhow::bail!("Not in a workspace directory. Run 'aoc-cli init <name>' first.");
    }

    println!("Running analytics on all day crates...\n");

    let mut timings = Vec::new();

    for day in 1..=25 {
        let day_name = format!("day{day:02}");
        let day_path = Path::new(&day_name);

        if !day_path.exists() {
            continue;
        }

        println!("Running {day_name}...");

        let output = Command::new("cargo")
            .args(["run", "--release", "-p", &day_name, "--", "--timing"])
            .output()
            .context(format!("Failed to run {day_name}"))?;

        if !output.status.success() {
            eprintln!("Warning: {day_name} failed to run, skipping...");
            continue;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        if let Some(timing) = parse_timing(&stdout, day) {
            println!(
                "  Part 1: {:.3}ms | Part 2: {:.3}ms | Total: {:.3}ms\n",
                timing.part1_ms(),
                timing.part2_ms(),
                timing.total_ms()
            );
            timings.push(timing);
        } else {
            eprintln!("Warning: Could not parse timing for {day_name}, skipping...\n");
        }
    }

    if timings.is_empty() {
        anyhow::bail!("No day crates found or all failed to run");
    }

    let table = format_table(&timings);
    fs::write(file_path, table).context("Failed to write analytics file")?;

    println!("Analytics written to {file_path}");

    Ok(())
}

fn parse_timing(output: &str, day: u8) -> Option<DayTiming> {
    for line in output.lines() {
        if let Some(timing_str) = line.strip_prefix("TIMING:") {
            let parts: Vec<&str> = timing_str.split(':').collect();
            if parts.len() == 2 {
                let part1_micros = parts[0].parse().ok()?;
                let part2_micros = parts[1].parse().ok()?;
                return Some(DayTiming {
                    day,
                    part1_micros,
                    part2_micros,
                });
            }
        }
    }
    None
}

fn format_table(timings: &[DayTiming]) -> String {
    use std::fmt::Write;

    let mut table = String::from("| day | part 1  | part 2  | total   |\n");
    table.push_str("| --- | ------- | ------- | ------- |\n");

    for timing in timings {
        let _ = writeln!(
            &mut table,
            "| {:>3} | {:>6.2}ms | {:>6.2}ms | {:>6.2}ms |",
            timing.day,
            timing.part1_ms(),
            timing.part2_ms(),
            timing.total_ms()
        );
    }

    table
}
