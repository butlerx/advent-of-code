//! Systems Check - Mission 3
//!
//! <https://systemscheck.dev/missions/3>

use std::collections::{HashSet, VecDeque};

use aoc_shared::Point;

fn parse_coord_line(line: &str) -> Point {
    let mut parts = line.split_whitespace().skip(1);
    let x = parts.next().and_then(|s| s.strip_prefix('X')?.parse().ok());
    let y = parts.next().and_then(|s| s.strip_prefix('Y')?.parse().ok());
    let (x, y): (i64, i64) = x.zip(y).expect("Invalid coordinate line");
    Point::new(x, y)
}

fn parse_input(input: &str) -> (Vec<Point>, Vec<usize>) {
    let (coords_str, path_str) = input
        .trim()
        .split_once("\n\n")
        .expect("Input must contain coordinates and path separated by a blank line");
    let coords = coords_str.lines().map(parse_coord_line).collect();
    let path = path_str
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().expect("Failed to parse path index"))
        .collect();
    (coords, path)
}

fn build_adjacency(coords: &[Point]) -> Vec<Vec<usize>> {
    (0..coords.len())
        .map(|i| {
            (0..coords.len())
                .filter(|&j| i != j && coords[i].manhattan_distance(coords[j]) <= 5)
                .collect()
        })
        .collect()
}

fn count_paths(adj: &[Vec<usize>], current: usize, dest: usize, visited: &mut [bool]) -> usize {
    if current == dest {
        return 1;
    }
    adj[current]
        .iter()
        .filter(|&&n| !visited[n])
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|neighbor| {
            visited[neighbor] = true;
            let paths = count_paths(adj, neighbor, dest, visited);
            visited[neighbor] = false;
            paths
        })
        .sum()
}

fn find_recipient(
    candidates: &[usize],
    rr: usize,
    chunk: usize,
    received: &[HashSet<usize>],
) -> Option<(usize, usize)> {
    (0..candidates.len())
        .map(|offset| (rr + offset) % candidates.len())
        .find(|&idx| !received[candidates[idx]].contains(&chunk))
        .map(|idx| (candidates[idx], (idx + 1) % candidates.len()))
}

fn forward_chunks(
    node: usize,
    dest: usize,
    candidates: &[usize],
    queues: &mut [VecDeque<usize>],
    received: &mut [HashSet<usize>],
) -> Vec<usize> {
    let chunks: Vec<usize> = queues[node].drain(..).collect();
    let (unsent, _, arrived) = chunks.into_iter().fold(
        (VecDeque::new(), 0, Vec::new()),
        |(mut unsent, rr, mut arrived), chunk| {
            if let Some((target, next_rr)) = find_recipient(candidates, rr, chunk, received) {
                queues[target].push_back(chunk);
                received[target].insert(chunk);
                if target == dest {
                    arrived.push(chunk);
                }
                (unsent, next_rr, arrived)
            } else {
                unsent.push_back(chunk);
                (unsent, rr, arrived)
            }
        },
    );
    queues[node] = unsent;
    arrived
}

fn next_forwarder(
    n: usize,
    dest: usize,
    queues: &[VecDeque<usize>],
    adj: &[Vec<usize>],
    received: &[HashSet<usize>],
) -> Option<usize> {
    (0..n)
        .filter(|&node| node != dest && !queues[node].is_empty())
        .find(|&node| {
            queues[node]
                .iter()
                .any(|&chunk| adj[node].iter().any(|&nb| !received[nb].contains(&chunk)))
        })
}

#[must_use]
pub fn part_1(input: &str) -> usize {
    let (coords, _) = parse_input(input);
    let adj = build_adjacency(&coords);
    let dest = coords.len() - 1;
    let mut visited = vec![false; coords.len()];
    visited[0] = true;
    count_paths(&adj, 0, dest, &mut visited)
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let (coords, message) = parse_input(input);
    let adj = build_adjacency(&coords);

    let n = coords.len();
    let dest = n - 1;

    let mut queues: Vec<VecDeque<usize>> = vec![VecDeque::new(); n];
    let mut received: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    for chunk in message {
        queues[0].push_back(chunk);
        received[0].insert(chunk);
    }

    let mut d_arrival_order: Vec<usize> = Vec::new();

    while let Some(node) = next_forwarder(n, dest, &queues, &adj, &received) {
        let mut candidates: Vec<usize> = adj[node].clone();
        candidates.sort_by_key(|&nb| {
            (
                coords[node].manhattan_distance(coords[nb]),
                queues[nb].len(),
                nb,
            )
        });

        let arrived = forward_chunks(node, dest, &candidates, &mut queues, &mut received);
        d_arrival_order.extend(arrived);
    }

    let total = d_arrival_order.len();
    let last_5_sum: usize = d_arrival_order.iter().rev().take(5).sum();
    last_5_sum * total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#S X1 Y0
#1 X5 Y1
#2 X8 Y2
#3 X9 Y6
#D X9 Y9

4,5,2,3,1";

    const TEST_INPUT_2: &str = "#S X1 Y0
#1 X3 Y3
#2 X5 Y1
#3 X8 Y2
#4 X9 Y6
#5 X10 Y1
#D X9 Y9

9,10,5,3,4,1,8,2,6,7";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 1);
        assert_eq!(part_1(TEST_INPUT_2), 4);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 75);
        assert_eq!(part_2(TEST_INPUT_2), 168);
    }
}
