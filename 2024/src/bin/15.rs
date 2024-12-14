#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::time_execution;
static INPUT_TXT: &str = include_str!("../../input/15.txt");

fn main() {
    println!("🌟 --- Day 15 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn part_1(_input: &str) -> u32 {
    0
}

fn part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "";
    static INPUT_2: &str = "";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 0);
        assert_eq!(part_1(INPUT_TXT), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_2), 0);
        assert_eq!(part_2(INPUT_TXT), 0);
    }
}
