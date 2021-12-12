use std::collections::HashMap;

static START: &str = "start";
static END: &str = "end";

fn main() {
    let input = include_str!("../../input/12.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    traverse(&parse_graph(input), START, &mut Vec::new(), true)
}

fn part_2(input: &str) -> usize {
    traverse(&parse_graph(input), START, &mut Vec::new(), false)
}

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_insert_with(Vec::new).push(b);
        graph.entry(b).or_insert_with(Vec::new).push(a);
    }
    graph
}

fn traverse<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current_pos: &'a str,
    path: &mut Vec<&'a str>,
    seen_twice: bool,
) -> usize {
    if current_pos == END {
        1
    } else if current_pos.chars().all(|c| c.is_lowercase()) && path.contains(&current_pos) {
        if seen_twice || current_pos == START {
            0
        } else {
            num_paths(graph, current_pos, path, true)
        }
    } else {
        num_paths(graph, current_pos, path, seen_twice)
    }
}

fn num_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current_pos: &'a str,
    path: &mut Vec<&'a str>,
    seen_twice: bool,
) -> usize {
    path.push(current_pos);
    let possible_paths = graph[current_pos]
        .iter()
        .map(|n| traverse(graph, n, path, seen_twice))
        .sum();
    path.pop();
    possible_paths
}

#[cfg(test)]
mod day_12_tests {
    use super::*;
    static INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    static MED_INPUT: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    static LARGE_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 10);
        assert_eq!(part_1(MED_INPUT), 19);
        assert_eq!(part_1(LARGE_INPUT), 226);
        assert_eq!(part_1(include_str!("../../input/12.txt")), 4707);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 36);
        assert_eq!(part_2(MED_INPUT), 103);
        assert_eq!(part_2(LARGE_INPUT), 3509);
        assert_eq!(part_2(include_str!("../../input/12.txt")), 130493);
    }
}
