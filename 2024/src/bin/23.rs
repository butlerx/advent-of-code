use aoc_shared::time_execution;
use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/23.txt");

fn main() {
    println!("🌟 --- Day 23 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

type Graph = HashMap<String, HashSet<String>>;

fn load_graph(input: &str) -> Graph {
    input
        .trim()
        .lines()
        .filter_map(|line| line.split_once('-'))
        .fold(HashMap::new(), |mut graph, (a, b)| {
            graph
                .entry(a.to_string())
                .or_default()
                .insert(b.to_string());
            graph
                .entry(b.to_string())
                .or_default()
                .insert(a.to_string());
            graph
        })
}

fn count_triangles(graph: &Graph) -> usize {
    let keys = &graph.keys().collect::<Vec<_>>();
    keys.iter()
        .enumerate()
        .flat_map(move |(i, a)| {
            keys.iter()
                .enumerate()
                .skip(i + 1)
                .filter(|(_, b)| graph[*a].contains(**b))
                .flat_map(move |(j, b)| keys.iter().skip(j + 1).map(move |c| (*a, *b, *c)))
        })
        .filter(|(_, b, c)| graph[*b].contains(*c))
        .filter(|(a, _, c)| graph[*c].contains(*a))
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count()
}

fn part_1(input: &str) -> usize {
    let graph = load_graph(input);
    count_triangles(&graph)
}

fn build_group(node: &str, neighbors: &HashSet<String>, graph: &Graph) -> Vec<String> {
    let mut group = vec![node.to_string()];
    for neighbor in neighbors {
        if group
            .iter()
            .all(|n| graph.get(neighbor).unwrap().contains(n))
        {
            group.push(neighbor.clone());
        }
    }
    group
}

fn part_2(input: &str) -> String {
    let graph = load_graph(input);
    graph
        .keys()
        .cloned()
        .collect::<HashSet<_>>()
        .iter()
        .filter_map(|node| {
            graph
                .get(node)
                .map(|neighbors| build_group(node, neighbors, &graph))
        })
        .max_by_key(Vec::len)
        .map(|mut group| {
            group.sort();
            group
        })
        .expect("No group found")
        .iter()
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 7);
        assert_eq!(part_1(INPUT_TXT), 1306);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), "co,de,ka,ta");
        assert_eq!(part_2(INPUT_TXT), "bd,dk,ir,ko,lk,nn,ob,pt,te,tl,uh,wj,yl");
    }
}
