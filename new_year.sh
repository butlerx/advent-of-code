#!/usr/bin/env bash
set -euo pipefail

YEAR="${1:-$(date +%Y)}"
BASE_DIR="$(dirname "$0")"
YEAR_DIR="$BASE_DIR/$YEAR"

if [ -d "$YEAR_DIR" ]; then
    echo "Year $YEAR already exists at $YEAR_DIR"
    exit 1
fi

# Create directory structure
mkdir -p "$YEAR_DIR/src/bin"
mkdir -p "$YEAR_DIR/input"

for i in $(seq -w 1 25); do
    cat <<EOF >"$YEAR_DIR/src/bin/${i}.rs"
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/${i}.txt");

fn main() {
    println!("🌟 --- Day 1 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn part_1(_input: &str) -> usize {
    0
}

fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
        assert_eq!(part1(INPUT_TXT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
        assert_eq!(part2(INPUT_TXT), 0);
    }
}
EOF

    touch "$YEAR_DIR/input/${i}.txt"
done

cat <<EOF >"$YEAR_DIR/src/lib.rs"
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
EOF

cat <<EOF >"$YEAR_DIR/Cargo.toml"
[package]
name = "aoc_$YEAR"
version = "0.1.0"
edition = "2024"

[dependencies]
aoc_shared = { path = "../aoc_shared" }

EOF

echo "Created Advent of Code directory for $YEAR"
