use itertools::Itertools;
use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/03.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn priority(score: i64, v: &char) -> i64 {
    score
        + (match v {
            'a'..='z' => (*v as u8) - b'a' + 1,
            'A'..='Z' => (*v as u8) - b'A' + 27,
            _ => unreachable!(),
        }) as i64
}

fn part_1(input: &str) -> i64 {
    input.trim().lines().fold(0, |score, line| {
        let (start, end) = line
            .chars()
            .chunks(line.len() / 2)
            .into_iter()
            .map(|part| part.collect::<HashSet<char>>())
            .collect_tuple()
            .unwrap();
        start.intersection(&end).fold(0, priority) + score
    })
}

fn part_2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .chunks(3)
        .into_iter()
        .fold(0, |score, lines| {
            let mut set_lines = lines.map(|line| line.chars().collect::<HashSet<char>>());
            let mut s = set_lines.next().unwrap();
            for line in set_lines {
                s = s.intersection(&line).copied().collect();
            }
            priority(score, s.iter().next().unwrap())
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
