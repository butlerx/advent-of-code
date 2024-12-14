#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::time_execution;
use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/11.txt");

fn main() {
    println!("🌟 --- Day 11 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn parse_input(input: &str) -> HashMap<usize, usize> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .fold(HashMap::default(), |mut acc, val| {
            *acc.entry(val).or_default() += 1;
            acc
        })
}

fn blink(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![1];
    }
    let num_len = n.ilog10() + 1;
    if num_len % 2 == 0 {
        let divisor = 10usize.pow(num_len / 2);
        vec![n / divisor, n % divisor]
    } else {
        vec![n * 2024]
    }
}

fn blink_all(numbers: HashMap<usize, usize>, times: usize) -> usize {
    (0..times)
        .fold(numbers, |acc, _| {
            acc.iter()
                .flat_map(|(n, freq)| blink(*n).into_iter().map(move |new_num| (new_num, freq)))
                .fold(HashMap::default(), |mut res, (n, freq)| {
                    *res.entry(n).or_default() += freq;
                    res
                })
        })
        .values()
        .sum()
}

fn part_1(input: &str) -> usize {
    blink_all(parse_input(input), 25)
}

fn part_2(input: &str) -> usize {
    blink_all(parse_input(input), 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "125 17";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 55312);
        assert_eq!(part_1(INPUT_TXT), 199_986);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 65_601_038_650_482);
        assert_eq!(part_2(INPUT_TXT), 236_804_088_748_754);
    }
}
