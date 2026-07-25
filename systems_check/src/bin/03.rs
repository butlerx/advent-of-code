//! Systems Check - Mission 3
//!
//! <https://systemscheck.dev/missions/3>
#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/03.txt");

fn main() {
    println!("🌟 --- Mission 3 Results --- 🌟");
    let (res, duration) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res}, complete in {duration} ms");
    let (res, duration) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res}, complete in {duration} ms");
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
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 0);
    }
}
