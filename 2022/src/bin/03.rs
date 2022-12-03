use itertools::Itertools;
use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/03.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn part_1(input: &str) -> i64 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect::<Vec<char>>();
    input.trim().lines().fold(0i64, |score, line| {
        let start = line.chars().take(line.len() / 2).collect::<HashSet<char>>();
        let end = line.chars().skip(line.len() / 2).collect::<HashSet<char>>();
        start.intersection(&end).fold(0i64, |count, letter| {
            count + (alphabet.iter().position(|&r| r == *letter).unwrap() as i64) + 1
        }) + score
    })
}

fn part_2(input: &str) -> i64 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect::<Vec<char>>();
    input
        .trim()
        .lines()
        .chunks(3)
        .into_iter()
        .fold(0i64, |score, lines| {
            let mut set_lines = lines.map(|line| line.chars().collect::<HashSet<char>>());
            let mut s = set_lines.next().unwrap();
            for line in set_lines {
                s = s.intersection(&line).copied().collect();
            }
            score
                + 1
                + (alphabet
                    .iter()
                    .position(|&r| r == *s.iter().next().unwrap())
                    .unwrap() as i64)
        })
}

#[cfg(test)]
mod day_3_tests {
    use super::*;
    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 157);
        assert_eq!(part_1(INPUT_TXT), 7742);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 70);
        assert_eq!(part_2(INPUT_TXT), 2276);
    }
}
