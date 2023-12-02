use itertools::{iproduct, Itertools};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

const EXTENT: std::ops::Range<i64> = 0..512;
static INPUT_TXT: &str = include_str!("../../input/06.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: &str) -> i64 {
    let positions = parse_input(input);
    let grid: Vec<Vec<_>> = EXTENT
        .map(|y| {
            EXTENT
                .map(|x| {
                    match positions.iter().fold(
                        ((0, 0), std::i64::MAX, false),
                        |nearest, &position| {
                            let distance = (x - position.0).abs() + (y - position.1).abs();

                            match distance.cmp(&nearest.1) {
                                Ordering::Less => (position, distance, false),
                                Ordering::Equal => (position, distance, true),
                                Ordering::Greater => nearest,
                            }
                        },
                    ) {
                        (position, _, false) => Some(position),
                        (_, _, true) => None,
                    }
                })
                .collect()
        })
        .collect();

    let infinites: HashSet<_> = grid[0]
        .iter()
        .chain(grid[grid.len() - 1].iter())
        .chain(grid.iter().map(|row| &row[0]))
        .chain(grid.iter().map(|row| &row[row.len() - 1]))
        .flatten()
        .collect();

    *grid
        .iter()
        .flatten()
        .flatten()
        .fold(HashMap::new(), |mut histogram, nearest| {
            if !infinites.contains(nearest) {
                *histogram.entry(nearest).or_insert(0) += 1;
            }
            histogram
        })
        .values()
        .max()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let positions = parse_input(input);
    iproduct!(EXTENT, EXTENT)
        .map(|a| {
            positions
                .iter()
                .fold(0, |sum, &b| sum + (a.0 - b.0).abs() + (a.1 - b.1).abs())
        })
        .filter(|&distance| distance < 10000)
        .count()
}

#[cfg(test)]
mod day_6_tests {
    use super::*;
    static INPUT: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 17);
        assert_eq!(part_1(INPUT_TXT), 3894);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 262_144);
        assert_eq!(part_2(INPUT_TXT), 39398);
    }
}
