#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::{time_execution, time_execution_us};
use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/11.txt");

fn main() {
    println!("🌟 --- Day 11 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} us");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &str) -> Graph<'_> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .map(|(key, values_str)| (key, values_str.split_whitespace().collect()))
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let devices = parse_input(input);
    count_paths(&devices, "you", "out", &HashSet::new())
}

fn count_paths<'a>(
    graph: &Graph<'a>,
    current: &'a str,
    target: &'a str,
    visited: &HashSet<&'a str>,
) -> usize {
    if current == target {
        return 1;
    }

    graph.get(current).map_or(0, |neighbors| {
        let new_visited = visited.iter().copied().chain([current]).collect();

        neighbors
            .iter()
            .filter(|&&n| !visited.contains(n))
            .map(|&n| count_paths(graph, n, target, &new_visited))
            .sum()
    })
}

fn part_2(input: &str) -> usize {
    let devices = parse_input(input);
    let required: HashSet<&str> = HashSet::from(["dac", "fft"]);

    count_paths_with_required(
        &devices,
        "svr",
        "out",
        &HashSet::new(),
        &HashSet::new(),
        &required,
        &mut HashMap::new(),
    )
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct MemoKey {
    node: String,
    seen: Vec<String>,
}

impl MemoKey {
    fn new(node: &str, seen: &HashSet<String>) -> Self {
        let mut seen_sorted: Vec<_> = seen.iter().cloned().collect();
        seen_sorted.sort_unstable();
        Self {
            node: node.to_string(),
            seen: seen_sorted,
        }
    }
}

fn count_paths_with_required<'a>(
    graph: &Graph<'a>,
    current: &'a str,
    target: &'a str,
    visited: &HashSet<&'a str>,
    seen_required: &HashSet<String>,
    required: &HashSet<&'a str>,
    memo: &mut HashMap<MemoKey, usize>,
) -> usize {
    let memo_key = MemoKey::new(current, seen_required);

    if let Some(&cached) = memo.get(&memo_key) {
        return cached;
    }

    let updated_seen = if required.contains(current) {
        seen_required
            .iter()
            .cloned()
            .chain([current.to_string()])
            .collect()
    } else {
        seen_required.clone()
    };

    if current == target {
        return usize::from(updated_seen.len() == required.len());
    }

    let result = graph.get(current).map_or(0, |neighbors| {
        let new_visited: HashSet<_> = visited.iter().copied().chain([current]).collect();

        neighbors
            .iter()
            .filter(|&&n| !visited.contains(n))
            .map(|&n| {
                count_paths_with_required(
                    graph,
                    n,
                    target,
                    &new_visited,
                    &updated_seen,
                    required,
                    memo,
                )
            })
            .sum()
    });

    memo.insert(memo_key, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 5);
        assert_eq!(part_1(INPUT_TXT), 772);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT_2), 2);
        assert_eq!(part_2(INPUT_TXT), 423_227_545_768_872);
    }
}
