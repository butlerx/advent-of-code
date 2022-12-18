use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};
static INPUT_TXT: &str = include_str!("../../input/05.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> (BTreeMap<usize, VecDeque<char>>, &str) {
    let (crates, moves) = input.split_once("\n\n").unwrap();
    let stacks: BTreeMap<usize, VecDeque<char>> =
        crates.lines().fold(BTreeMap::new(), |mut stacks, line| {
            if !line.contains('[') {
                return stacks;
            }
            for (i, c) in line
                .chars()
                .enumerate()
                .filter(|(i, _)| i % 4 == 1)
                .map(|(_, v)| v)
                .enumerate()
            {
                if !c.is_whitespace() {
                    stacks.entry(i + 1).or_default().push_front(c);
                }
            }
            stacks
        });
    (stacks, moves.trim())
}

fn parse_move(line: &str) -> (usize, usize, usize) {
    line.split_whitespace()
        .chunks(2)
        .into_iter()
        .map(|mut s| s.nth(1).unwrap().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part_1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for line in moves.lines() {
        let (num_to_move, source, dest) = parse_move(line);
        for _ in 0..num_to_move {
            let v = stacks.get_mut(&source).unwrap().pop_back().unwrap();
            stacks.get_mut(&dest).unwrap().push_back(v);
        }
    }
    stacks.values().map(|v| v.back().unwrap()).collect()
}

fn part_2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for line in moves.trim().lines() {
        let (num_to_move, source, dest) = parse_move(line);
        let crates = (0..num_to_move)
            .map(|_| stacks.get_mut(&source).unwrap().pop_back().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev();
        stacks.get_mut(&dest).unwrap().extend(crates);
    }
    stacks.values().map(|v| v.back().unwrap()).collect()
}

#[cfg(test)]
mod day_5_tests {
    use super::*;
    static INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), "CMZ");
        assert_eq!(part_1(INPUT_TXT), "HBTMTBSDC");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), "MCD");
        assert_eq!(part_2(INPUT_TXT), "PQTJRSHWS");
    }
}
