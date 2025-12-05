#![warn(clippy::pedantic, clippy::perf)]
use std::{collections::HashMap, mem::swap};

static INPUT_TXT: &str = include_str!("../../input/08.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

enum Choice {
    Left,
    Right,
}

impl From<char> for Choice {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Choice>, HashMap<&str, (&str, &str)>) {
    let (dir_str, map_str) = input.trim().split_once("\n\n").unwrap();
    let directions = dir_str.trim().chars().map(Choice::from).collect::<Vec<_>>();
    let map = map_str
        .lines()
        .map(|l| {
            l.split_once(" = ")
                .map(|(k, v)| {
                    let value = v
                        .split_once(", ")
                        .map(|s| (s.0.trim_matches('('), s.1.trim_matches(')')))
                        .expect("Invalid tuple");
                    (k, value)
                })
                .expect("Invalid map")
        })
        .collect::<HashMap<_, _>>();
    (directions, map)
}

fn traverse(
    directions: &[Choice],
    map: &HashMap<&str, (&str, &str)>,
    start_position: &str,
    ends_with: &str,
) -> usize {
    let mut pos = start_position;
    let mut count = 0;
    for choice in directions.iter().cycle() {
        let next = map.get(pos).unwrap();
        count += 1;
        pos = match choice {
            Choice::Left => next.0,
            Choice::Right => next.1,
        };
        if pos.ends_with(ends_with) {
            break;
        }
    }
    count
}

fn part_1(input: &str) -> usize {
    let (directions, map) = parse_input(input);
    traverse(&directions, &map, "AAA", "ZZZ")
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            break min;
        }

        max = min;
        min = res;
    }
}

fn part_2(input: &str) -> usize {
    let (directions, map) = parse_input(input);
    map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|pos| traverse(&directions, &map, pos, "Z"))
        .fold(1, |a, b| a * b / gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    static INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT_1), 2);
        assert_eq!(part_1(INPUT_2), 6);
        assert_eq!(part_1(INPUT_TXT), 12169);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_3), 6);
        assert_eq!(part_2(INPUT_TXT), 12_030_780_859_469);
    }
}
