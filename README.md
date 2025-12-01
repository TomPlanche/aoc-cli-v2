# AOC CLI

A command-line interface tool for scaffolding Advent of Code projects in Rust. Designed to eliminate boilerplate and accelerate daily challenge setup through workspace-based project generation with shared utilities.

## Overview

AOC CLI streamlines your Advent of Code workflow by generating consistent, well-structured Rust projects. Each project includes a shared utilities crate with common helpers for grid navigation, point mathematics, and a standardized solution trait pattern.

### Key Features

- **Workspace Generation**: Creates cargo workspaces with shared utilities and individual day crates
- **Zero Boilerplate**: Pre-configured templates with parsing separation and type-safe patterns
- **Coordinate System**: Comprehensive (x, y) coordinate utilities with 8-directional movement
- **Generic Point Types**: Flexible point implementation supporting all numeric types
- **Solution Trait Pattern**: Structured approach separating input parsing from solving logic
- **Single Binary Distribution**: Templates embedded at compile-time for zero dependencies

## Installation

### From Source

```bash
git clone https://github.com/yourusername/aoc-cli-v2.git
cd aoc-cli-v2
cargo build --release
```

The compiled binary will be available at `./target/release/aoc-cli-v2`.

### Add to PATH

For convenient access from anywhere:

```bash
# Linux/macOS
cp ./target/release/aoc-cli-v2 ~/.local/bin/aoc-cli

# Or create an alias
alias aoc='/path/to/aoc-cli-v2/target/release/aoc-cli-v2'
```

## Quick Start

### Initialize a New Project

```bash
aoc-cli-v2 init aoc2025
cd aoc2025
```

This creates a workspace with:
- Root `Cargo.toml` configured for workspace members
- `utils/` crate containing Direction, Point, and Solution trait
- Ready for day crate additions

### Add Daily Challenges

```bash
# Add day 1
aoc-cli-v2 add 1

# Add day 15
aoc-cli-v2 add 15
```

Each day includes:
- Pre-configured `Cargo.toml` with utils dependency
- `main.rs` implementing the Solution trait pattern
- Empty `input.txt` for challenge input

### Solve a Challenge

```rust
// day01/src/main.rs
use utils::{run_solution, Solution};

struct Day01;

impl Solution for Day01 {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        data.iter().sum()
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        data.iter().product()
    }
}

fn main() {
    run_solution!(Day01);
}
```

Run your solution:

```bash
cd day01
# Add your puzzle input to input.txt
cargo run
```

## Project Architecture

### Template System

Templates are embedded at compile-time using `include_str!()`, ensuring:
- Single binary distribution without external dependencies
- Immutable templates requiring recompilation for changes
- Type-safe template loading

### Source Structure

```
src/
├── main.rs                    # CLI parsing and command dispatch
├── templates.rs               # Embedded template constants
└── commands/
    ├── mod.rs                # Module exports
    ├── init.rs               # Workspace initialization
    └── add.rs                # Day scaffolding
```

### Generated Project Structure

```
aoc2025/
├── Cargo.toml                 # Workspace configuration
├── utils/                     # Shared utilities crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Solution trait and macros
│       ├── directions.rs     # 8-directional movement
│       └── points.rs         # Generic Point<T> implementation
├── day01/
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── input.txt
├── day02/
│   └── ...
```

## Utilities Reference

### Solution Trait

Provides structured approach to daily challenges:

```rust
pub trait Solution {
    type Input;
    type Output: Display;

    fn parse_input(&self, input: &str) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> Self::Output;
    fn part2(&self, input: &Self::Input) -> Self::Output;
}
```

Benefits:
- Parsing separated from solving logic
- Input parsed once, used for both parts
- Generic types for any input/output structure
- Automatic file reading via `run_solution!` macro

### Direction Enum

8-directional movement with (x, y) coordinate system:

```rust
pub enum Direction {
    Up, Down, Left, Right,
    UpLeft, UpRight, DownLeft, DownRight,
}

// Convert to deltas
let (dx, dy): (isize, isize) = Direction::Up.into();  // (0, -1)

// Add to positions
let new_pos = (5, 10) + Direction::Right;  // (6, 10)

// Utility methods
direction.x_delta();           // Get x component
direction.y_delta();           // Get y component
direction.turn_clockwise();    // Rotate 90° CW
```

### Point Type

Generic point implementation supporting all numeric types:

```rust
let p1 = Point::new(10, 20);
let p2 = Point::new(15, 25);

// Arithmetic operations
let sum = p1 + p2;
let diff = p2 - p1;

// Manhattan distance
let dist = p1.manhattan_distance(&p2);  // 10

// Type flexibility
let point_i32: Point<i32> = Point::new(5, 10);
let point_f64: Point<f64> = Point::new(5.5, 10.5);
```

## Development

### Building

```bash
cargo build --release
```

### Code Quality

Run quality checks before committing:

```bash
cargo clippy --workspace --release --all-targets --all-features -- \
  --deny warnings -D warnings \
  -W clippy::correctness \
  -W clippy::suspicious \
  -W clippy::complexity \
  -W clippy::perf \
  -W clippy::style \
  -W clippy::pedantic

cargo fmt --all -- --check
```

### Modifying Templates

1. Edit template files in `templates/` directory
2. Rebuild with `cargo build --release` (templates are embedded at compile-time)
3. Test by creating a new project
4. Verify generated workspace builds: `cd test-project && cargo check`

## Design Decisions

### Coordinate System Convention

All utilities use **(x, y) coordinates**, not (row, col) or (y, x). This affects:
- Direction enum conversions to `(dx, dy)` tuples
- Point struct field ordering (`.x`, `.y`)
- Tuple operation ordering in Add implementations
- Direction delta methods (`x_delta()`, `y_delta()`)

When modifying direction or point logic, maintain (x, y) ordering consistently.

### Template Placeholders

The `{DAY}` placeholder is replaced during day creation:
- In `Cargo.toml`: Package name becomes `dayXX` (e.g., "day01", "day15")
- In `main.rs`: Struct name becomes `DayXX` (e.g., "Day01", "Day15")
- Uses zero-padded format via `format!("{day:02}")`

## Contributing

Contributions are welcome! Please ensure all code passes clippy and formatting checks before submitting.

## License

This project is open source. See LICENSE file for details.

## Support

For issues, questions, or feature requests, please open an issue on the GitHub repository.
