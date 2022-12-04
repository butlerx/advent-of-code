use itertools::Itertools;
use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/04.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let (a, b) = line
                .split(",")
                .map(|range| {
                    let (start, fininsh) = range.split_once("-").unwrap();
                    (
                        start.parse::<i64>().unwrap(),
                        fininsh.parse::<i64>().unwrap(),
                    )
                })
                .collect_tuple()
                .unwrap();
            ((a.0 <= b.0) && (a.1 >= b.1)) || ((b.0 <= a.0) && (b.1 >= a.1))
        })
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let (a, b) = line
                .split(",")
                .map(|range| {
                    let (start, fininsh) = range.split_once("-").unwrap();
                    (start.parse::<i64>().unwrap()..=fininsh.parse::<i64>().unwrap())
                        .collect::<HashSet<_>>()
                })
                .collect_tuple()
                .unwrap();
            a.intersection(&b).count() > 0
        })
        .count()
}

#[cfg(test)]
mod day_4_tests {
    use super::*;
    static INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 2);
        assert_eq!(part_1(INPUT_TXT), 518);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 4);
        assert_eq!(part_2(INPUT_TXT), 909);
    }
}
