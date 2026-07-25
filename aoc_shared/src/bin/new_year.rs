#![warn(clippy::pedantic, clippy::perf)]

//! Scaffolds a new Advent of Code year directory.
//!
//! Creates the standard directory layout with input files, library modules, and
//! Cargo configuration for a given year:
//!
//! ```text
//! <year>/
//! ├── Cargo.toml
//! ├── input/
//! │   ├── 01.txt ... 12.txt
//! └── src/
//!     ├── lib.rs
//!     ├── main.rs
//!     ├── day_01.rs ... day_12.rs
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

    let crate_name = format!("aoc_{year}");

    fs::create_dir_all(year_dir.join("src"))?;
    fs::create_dir_all(year_dir.join("input"))?;
    File::create(year_dir.join("input/.gitkeep"))?;

    write_day_modules(&year_dir, &year)?;
    write_lib(&year_dir)?;
    write_main(&year_dir, &crate_name)?;
    write_cargo_toml(&year_dir, &year, &crate_name)?;
    write_readme(&year_dir, &year, &crate_name)?;

    println!("Created Advent of Code directory for {year}");
    Ok(())
}

fn write_day_modules(year_dir: &std::path::Path, year: &str) -> std::io::Result<()> {
    for day in 1..=12 {
        let day_str = format!("{day:02}");
        let module_path = year_dir.join(format!("src/day_{day_str}.rs"));
        let input_path = year_dir.join(format!("input/{day_str}.txt"));

        let module_template = format!(
            r#"//! Advent of Code {year} - Day {day}
//!
//! <https://adventofcode.com/{year}/day/{day}>

pub fn part_1(_input: &str) -> usize {{
    0
}}

pub fn part_2(_input: &str) -> usize {{
    0
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part_1() {{
        assert_eq!(part_1(TEST_INPUT), 0);
    }}

    #[test]
    fn test_part_2() {{
        assert_eq!(part_2(TEST_INPUT), 0);
    }}
}}"#
        );

        let mut module_file = File::create(module_path)?;
        module_file.write_all(module_template.as_bytes())?;

        File::create(input_path)?;
    }
    Ok(())
}

fn write_lib(year_dir: &std::path::Path) -> std::io::Result<()> {
    let lib_path = year_dir.join("src/lib.rs");
    let lib_content = "#![warn(clippy::pedantic, clippy::perf)]\n\n".to_owned()
        + &(1..=12)
            .map(|day| format!("pub mod day_{day:02};"))
            .collect::<Vec<_>>()
            .join("\n")
        + "\n";
    let mut lib_file = File::create(lib_path)?;
    lib_file.write_all(lib_content.as_bytes())
}

fn write_main(year_dir: &std::path::Path, crate_name: &str) -> std::io::Result<()> {
    let main_path = year_dir.join("src/main.rs");
    let main_content = format!(
        r#"#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::{{read_input, time_execution, Args}};

type Solver = fn(&str) -> usize;

fn main() {{
    let args = Args::parse("{crate_name}");
    let input = read_input(env!("CARGO_MANIFEST_DIR"), args.day);

    let (part_1, part_2): (Solver, Solver) = match args.day {{
{match_arms}
        _ => panic!("Unknown day: {{}}", args.day),
    }};

    println!("🌟 --- Day {{}} Results --- 🌟", args.day);
    if args.part != Some(2) {{
        let (res, duration) = time_execution(|| part_1(&input));
        println!("📌 Part 1: {{res}}, complete in {{duration}} ms");
    }}
    if args.part != Some(1) {{
        let (res, duration) = time_execution(|| part_2(&input));
        println!("📌 Part 2: {{res}}, complete in {{duration}} ms");
    }}
}}"#,
        match_arms = (1..=12)
            .map(|day| format!(
                "        {day} => ({crate_name}::day_{day:02}::part_1, {crate_name}::day_{day:02}::part_2),"
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let mut main_file = File::create(main_path)?;
    main_file.write_all(main_content.as_bytes())
}

fn write_cargo_toml(
    year_dir: &std::path::Path,
    year: &str,
    crate_name: &str,
) -> std::io::Result<()> {
    let cargo_path = year_dir.join("Cargo.toml");
    let cargo_content = format!(
        r#"[package]
name = "{crate_name}"
description = "Advent of Code {year} solutions"
version = "0.1.0"
edition = "2024"
readme = "README.md"

[dependencies]
aoc_shared.workspace = true"#
    );
    let mut cargo_file = File::create(cargo_path)?;
    cargo_file.write_all(cargo_content.as_bytes())
}

fn write_readme(year_dir: &std::path::Path, year: &str, crate_name: &str) -> std::io::Result<()> {
    let readme_path = year_dir.join("README.md");
    let readme_content = format!(
        r"# Advent of Code {year}

Solutions for [Advent of Code {year}](https://adventofcode.com/{year}) in Rust.

## Structure

- `src/main.rs` — CLI entrypoint, dispatches to the selected day solver.
- `src/day_XX.rs` — Solution for each day (parts 1 and 2).
- `src/lib.rs` — Shared utilities for the year.
- `input/XX.txt` — Puzzle input for each day.

## Running

```sh
cargo run --package {crate_name} -- --day 1
```

Run a single part:

```sh
cargo run --package {crate_name} -- --day 1 --part 1
```

## Testing

```sh
cargo test --package {crate_name}
```
"
    );
    let mut readme_file = File::create(readme_path)?;
    readme_file.write_all(readme_content.as_bytes())
}
