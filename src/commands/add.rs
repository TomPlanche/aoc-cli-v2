use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::templates;

pub fn add_day(day: u8) -> Result<()> {
    if !(1..=25).contains(&day) {
        anyhow::bail!("Day must be between 1 and 25");
    }

    if !Path::new("Cargo.toml").exists() {
        anyhow::bail!("Not in a workspace directory. Run 'aoc-cli init <name>' first.");
    }

    let day_name = format!("day{day:02}");
    let day_path = PathBuf::from(&day_name);

    if day_path.exists() {
        anyhow::bail!("Day {day} already exists");
    }

    println!("Adding {day_name}...");

    fs::create_dir(&day_path).context(format!("Failed to create {day_name} directory"))?;

    let src_path = day_path.join("src");
    fs::create_dir(&src_path).context(format!("Failed to create {day_name}/src directory"))?;

    let cargo_content = templates::DAY_CARGO_TOML.replace("{DAY}", &format!("{day:02}"));
    fs::write(day_path.join("Cargo.toml"), cargo_content)
        .context(format!("Failed to write {day_name}/Cargo.toml"))?;

    let main_content = templates::DAY_MAIN_RS.replace("{DAY}", &format!("{day:02}"));
    fs::write(src_path.join("main.rs"), main_content)
        .context(format!("Failed to write {day_name}/src/main.rs"))?;

    fs::write(day_path.join("input.txt"), "")
        .context(format!("Failed to write {day_name}/input.txt"))?;

    let readme_content = templates::DAY_README.replace("{DAY}", &format!("{day:02}"));
    fs::write(day_path.join("README.md"), readme_content)
        .context(format!("Failed to write {day_name}/README.md"))?;

    println!("{day_name} added successfully!");
    println!("\nNew structure:");
    println!("  {day_name}/");
    println!("  ├── Cargo.toml");
    println!("  ├── README.md");
    println!("  ├── src/");
    println!("  │   └── main.rs");
    println!("  └── input.txt");

    Ok(())
}
