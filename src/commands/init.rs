use anyhow::{Context, Result};
use chrono::Datelike;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::templates;

fn extract_year_from_name(name: &str) -> String {
    // Try to extract a 4-digit year from the project name (e.g., "aoc2025" -> "2025")
    name.chars()
        .collect::<Vec<_>>()
        .windows(4)
        .find_map(|window| {
            let year_str: String = window.iter().collect();
            if year_str.chars().all(|c| c.is_ascii_digit())
                && let Ok(year) = year_str.parse::<u16>()
                && (2015..=2030).contains(&year)
            {
                return Some(year_str);
            }
            None
        })
        .unwrap_or_else(|| chrono::Utc::now().year().to_string())
}

pub fn init_project(name: &str) -> Result<()> {
    let project_path = PathBuf::from(name);

    if project_path.exists() {
        anyhow::bail!("Directory '{name}' already exists");
    }

    println!("Creating project '{name}'...");

    //  project root
    fs::create_dir(&project_path).context("Failed to create project directory")?;

    fs::write(
        project_path.join("Cargo.toml"),
        templates::WORKSPACE_CARGO_TOML,
    )
    .context("Failed to write workspace Cargo.toml")?;

    let year = extract_year_from_name(name);
    let readme_content = templates::WORKSPACE_README.replace("{YEAR}", &year);
    fs::write(project_path.join("README.md"), readme_content)
        .context("Failed to write workspace README.md")?;

    let utils_path = project_path.join("utils");
    fs::create_dir(&utils_path).context("Failed to create utils directory")?;

    let utils_src_path = utils_path.join("src");
    fs::create_dir(&utils_src_path).context("Failed to create utils/src directory")?;

    fs::write(utils_path.join("Cargo.toml"), templates::UTILS_CARGO_TOML)
        .context("Failed to write utils Cargo.toml")?;

    fs::write(utils_src_path.join("lib.rs"), templates::UTILS_LIB_RS)
        .context("Failed to write utils lib.rs")?;

    fs::write(
        utils_src_path.join("directions.rs"),
        templates::UTILS_DIRECTIONS_RS,
    )
    .context("Failed to write utils directions.rs")?;

    fs::write(utils_src_path.join("points.rs"), templates::UTILS_POINTS_RS)
        .context("Failed to write utils points.rs")?;

    fs::write(
        utils_src_path.join("point3d.rs"),
        templates::UTILS_POINT3D_RS,
    )
    .context("Failed to write utils point3d.rs")?;

    // Initialize git repository
    Command::new("git")
        .arg("init")
        .current_dir(&project_path)
        .output()
        .context("Failed to initialize git repository")?;

    println!("Project '{name}' initialized successfully!");
    println!("\nProject structure:");
    println!("  {name}/");
    println!("  ├── Cargo.toml");
    println!("  ├── README.md");
    println!("  └── utils/");
    println!("      ├── Cargo.toml");
    println!("      └── src/");
    println!("          ├── lib.rs");
    println!("          ├── directions.rs");
    println!("          ├── point3d.rs");
    println!("          └── points.rs");

    Ok(())
}
