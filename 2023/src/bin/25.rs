#![warn(clippy::pedantic, clippy::perf)]
use std::collections::{HashMap, HashSet, VecDeque};

static INPUT_TXT: &str = include_str!("../../input/25.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
}

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;
type Edge<'a> = (&'a str, &'a str);

fn parse_input(input: &str) -> Graph<'_> {
    let mut graph = HashMap::<_, HashSet<_>>::new();
    for l in input.trim().lines() {
        let (a, rest) = l.split_once(": ").unwrap();
        for b in rest.split_whitespace() {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        }
    }
    graph
}

fn calculate_frequencies<'a>(edges: &'a Graph<'a>) -> Vec<(Edge<'a>, usize)> {
    let mut freq = HashMap::new();

    for &start in edges.keys() {
        let mut todo = VecDeque::from([start]);
        let mut seen = HashSet::from([start]);

        while let Some(pos) = todo.pop_front() {
            for &next in &edges[pos] {
                if seen.insert(next) {
                    let key = if pos < next { (pos, next) } else { (next, pos) };

                    let entry = freq.entry(key).or_insert(0);
                    *entry += 1;

                    todo.push_back(next);
                }
            }
        }
    }

    let mut order = freq
        .iter()
        .map(|(edge, size)| (*edge, *size))
        .collect::<Vec<_>>();
    order.sort_unstable_by_key(|e| e.1);
    order.reverse();
    order
}

fn calculate_group_size(edges: &Graph, cut: &[Edge], start: &str) -> usize {
    let mut size = 1;
    let mut todo = VecDeque::from([start]);
    let mut seen = HashSet::from([start]);

    while let Some(pos) = todo.pop_front() {
        for &next in &edges[pos] {
            let key = if pos < next { (pos, next) } else { (next, pos) };
            if !cut.contains(&key) && seen.insert(next) {
                size += 1;
                todo.push_back(next);
            }
        }
    }

    size * (edges.len() - size)
}

fn part_1(input: &str) -> usize {
    let graph = parse_input(input);
    let cut: Vec<_> = calculate_frequencies(&graph)
        .iter()
        .take(3)
        .map(|p| p.0)
        .collect();
    let start = *graph.keys().next().unwrap();
    calculate_group_size(&graph, &cut, start)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 54);
        assert_eq!(part_1(INPUT_TXT), 559143);
    }
}
