use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::templates;

pub fn update_utils() -> Result<()> {
    if !Path::new("Cargo.toml").exists() {
        anyhow::bail!("Not in a workspace directory. Run 'aoc-cli init <name>' first.");
    }

    let utils_path = Path::new("utils");
    if !utils_path.exists() {
        anyhow::bail!("utils/ directory not found. This doesn't appear to be an AOC workspace.");
    }

    println!("Updating utils crate to latest version...\n");

    let files = [
        ("utils/Cargo.toml", templates::UTILS_CARGO_TOML),
        ("utils/src/lib.rs", templates::UTILS_LIB_RS),
        ("utils/src/directions.rs", templates::UTILS_DIRECTIONS_RS),
        ("utils/src/points.rs", templates::UTILS_POINTS_RS),
        ("utils/src/point3d.rs", templates::UTILS_POINT3D_RS),
    ];

    for (path, content) in &files {
        fs::write(path, content).context(format!("Failed to write {path}"))?;
        println!("Updated {path}");
    }

    println!("\nUtils crate updated successfully!");
    println!("You may need to rebuild your project: cargo build --release");

    Ok(())
}
