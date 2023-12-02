use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

static INPUT_TXT: &str = include_str!("../../input/07.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> HashMap<char, Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut chars = line.chars().skip(5);
            (chars.next().unwrap(), chars.nth(30).unwrap())
        })
        .into_group_map()
}

fn find_dependencies(graph: &HashMap<char, Vec<char>>) -> HashMap<char, i64> {
    graph.iter().fold(HashMap::new(), |mut deps, (a, b)| {
        deps.entry(*a).or_default();
        for &node in b {
            *deps.entry(node).or_default() += 1;
        }
        deps
    })
}

fn create_queue(dependencies: &HashMap<char, i64>) -> BinaryHeap<Reverse<char>> {
    dependencies
        .iter()
        .filter_map(|(&node, &count)| {
            if count == 0 {
                Some(Reverse(node))
            } else {
                None
            }
        })
        .collect()
}

fn part_1(input: &str) -> String {
    let graph = parse_input(input);
    let mut result = String::new();
    let mut dependencies = find_dependencies(&graph);
    let mut queue = create_queue(&dependencies);

    loop {
        match queue.pop() {
            Some(Reverse(node)) => {
                result.push(node);
                for child in graph.get(&node).iter().flat_map(|&v| v) {
                    match dependencies.get_mut(child) {
                        Some(1) | None => queue.push(Reverse(*child)),
                        Some(parents) => *parents -= 1,
                    }
                }
            }
            None => break result,
        }
    }
}

fn part_2(input: &str) -> i64 {
    let mut elapsed = 0;
    let mut workers: [(Option<char>, i64); 5] = [(None, 0); 5];
    let graph = parse_input(input);
    let mut dependencies = find_dependencies(&graph);
    let mut queue = create_queue(&dependencies);

    loop {
        if workers[0].1 > elapsed {
            elapsed = workers[0].1;
            if let Some(children) = workers[0].0.take().and_then(|n| graph.get(&n)) {
                for child in children {
                    match dependencies.get_mut(child) {
                        Some(1) | None => queue.push(Reverse(*child)),
                        Some(parents) => *parents -= 1,
                    }
                }
            }
        }

        if let Some(Reverse(node)) = queue.pop() {
            workers[0] = (Some(node), elapsed + node as i64 - 4);
            workers.sort_unstable_by_key(|w| w.1);
        } else if let Some(position) = workers.iter().position(|w| w.1 > elapsed) {
            workers.rotate_left(position);
        } else {
            break elapsed;
        }
    }
}

#[cfg(test)]
mod day_7_tests {
    use super::*;
    static INPUT: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), "CABDFE");
        assert_eq!(part_1(INPUT_TXT), "DFOQPTELAYRVUMXHKWSGZBCJIN");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 253);
        assert_eq!(part_2(INPUT_TXT), 1036);
    }
}
