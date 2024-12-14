#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::time_execution;
static INPUT_TXT: &str = include_str!("../../input/01.txt");

fn main() {
    println!("🌟 --- Day 1 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut col_a, mut col_b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once(char::is_whitespace)
                .expect("invalid line format");
            (
                a.trim().parse::<u32>().expect("not a valid number"),
                b.trim().parse::<u32>().expect("not a valid number"),
            )
        })
        .unzip();

    col_a.sort_unstable();
    col_b.sort_unstable();

    (col_a, col_b)
}

fn part_1(input: &str) -> u32 {
    let (col_a, col_b) = parse_input(input);
    col_a
        .iter()
        .zip(col_b.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let (col_a, col_b) = parse_input(input);
    let mut cache = std::collections::HashMap::new();
    col_a
        .iter()
        .map(|a| {
            *a * cache
                .entry(a)
                .or_insert_with(|| u32::try_from(col_b.iter().filter(|&b| b == a).count()))
                .expect("invalid number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 11);
        assert_eq!(part_1(INPUT_TXT), 2_000_468);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 31);
        assert_eq!(part_2(INPUT_TXT), 18_567_089);
    }
}
