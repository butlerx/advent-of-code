#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/21.txt");

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
