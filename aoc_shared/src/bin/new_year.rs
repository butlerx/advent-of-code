#![warn(clippy::pedantic, clippy::perf)]

//! Scaffolds a new Advent of Code year directory.
//!
//! Creates the standard directory layout with input files, binary templates, and
//! Cargo configuration for a given year:
//!
//! ```text
//! <year>/
//! ├── Cargo.toml
//! ├── input/
//! │   ├── 01.txt ... 12.txt
//! └── src/
//!     ├── lib.rs
//!     └── bin/
//!         ├── 01.rs ... 12.rs
//! ```
//!
//! # Usage
//!
//! ```sh
//! cargo run --package aoc_shared --bin new_year [YEAR]
//! ```
//!
//! If `YEAR` is omitted, the current calendar year is used.

use std::{
    env,
    fs::{self, File},
    io::Write,
    process,
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let year = if args.len() > 1 {
        args[1].clone()
    } else {
        chrono::Local::now().format("%Y").to_string()
    };

    let base_dir = env::current_dir()?;
    let year_dir = base_dir.join(&year);

    if year_dir.exists() {
        eprintln!("Year {} already exists at {}", year, year_dir.display());
        process::exit(1);
    }

    // Create directory structure
    fs::create_dir_all(year_dir.join("src/bin"))?;
    fs::create_dir_all(year_dir.join("input"))?;

    // Create 12 day files
    for day in 1..=12 {
        let day_str = format!("{day:02}");
        let bin_path = year_dir.join(format!("src/bin/{day_str}.rs"));
        let input_path = year_dir.join(format!("input/{day_str}.txt"));

        let rust_template = format!(
            r#"//! Advent of Code {year} - Day {day}
//!
//! <https://adventofcode.com/{year}/day/{day}>
#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/{day_str}.txt");

fn main() {{
    println!("🌟 --- Day {day} Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {{res_1}}, complete in {{duration_1}} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {{res_2}}, complete in {{duration_2}} ms");
}}

fn part_1(_input: &str) -> usize {{
    0
}}

fn part_2(_input: &str) -> usize {{
    0
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {{
        assert_eq!(part_1(TEST_INPUT), 0);
    }}

    #[test]
    fn test_part2() {{
        assert_eq!(part_2(TEST_INPUT), 0);
    }}
}}"#
        );

        let mut bin_file = File::create(bin_path)?;
        bin_file.write_all(rust_template.as_bytes())?;

        File::create(input_path)?;
    }

    // Create lib.rs
    let lib_path = year_dir.join("src/lib.rs");
    let lib_content = r"#![warn(clippy::pedantic, clippy::perf)]";
    let mut lib_file = File::create(lib_path)?;
    lib_file.write_all(lib_content.as_bytes())?;

    // Create Cargo.toml (use edition = "2021" instead of "2024")
    let cargo_path = year_dir.join("Cargo.toml");
    let cargo_content = format!(
        r#"[package]
name = "aoc_{year}"
description = "Advent of Code {year} solutions"
version = "0.1.0"
edition = "2024"

[dependencies]
aoc_shared.workspace = true"#
    );
    let mut cargo_file = File::create(cargo_path)?;
    cargo_file.write_all(cargo_content.as_bytes())?;

    println!("Created Advent of Code directory for {year}");
    Ok(())
}
