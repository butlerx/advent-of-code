#![warn(clippy::pedantic, clippy::perf)]
use aoc_shared::time_execution;
use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/22.txt");

fn main() {
    println!("🌟 --- Day 22 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn mix<T: std::ops::BitXor<Output = T>>(a: T, b: T) -> T {
    a ^ b
}

fn prune<T: std::ops::Rem<i64, Output = T>>(a: T) -> T {
    a % 16_777_216
}

fn step(number: i64) -> i64 {
    let a = prune(mix(number, number * 64));
    let b = prune(mix(a, a / 32));
    prune(mix(b, b * 2048))
}

fn parse_input(input: &str) -> impl Iterator<Item = i64> + '_ {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<i64>().expect("invalid number"))
}

fn part_1(input: &str) -> i64 {
    parse_input(input)
        .map(|n| (0..2000).fold(n, |number, _| step(number)))
        .sum()
}

fn part_2(input: &str) -> i64 {
    *parse_input(input)
        .map(|init| {
            std::iter::once(init)
                .chain((0..2000).scan(init, |state, _| {
                    *state = step(*state);
                    Some(*state)
                }))
                .map(|x| x % 10)
                .collect::<Vec<_>>()
        })
        .flat_map(|values| {
            let mut seen = HashSet::with_capacity(2000);
            (0..values.len() - 4).filter_map(move |i| {
                let key = (values[i + 1] - values[i])
                    + (values[i + 2] - values[i + 1]) * 100
                    + (values[i + 3] - values[i + 2]) * 10_000
                    + (values[i + 4] - values[i + 3]) * 1_000_000;

                seen.insert(key).then_some((key, values[i + 4]))
            })
        })
        .fold(HashMap::with_capacity(1024), |mut result_map, (k, v)| {
            *result_map.entry(k).or_default() += v;
            result_map
        })
        .values()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "1
10
100
2024";
    static INPUT_2: &str = "1
2
3
2024";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 37_327_623);
        assert_eq!(part_1(INPUT_TXT), 18_261_820_068);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_2), 23);
        assert_eq!(part_2(INPUT_TXT), 2044);
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100_000_000), 16_113_920);
    }

    #[test]
    fn test_step() {
        assert_eq!(step(123), 15_887_950);
    }

    #[test]
    fn test_folded_step() {
        let n = (0..10).fold(123, |number, _| step(number));
        assert_eq!(n, 5_908_254);
    }
}
