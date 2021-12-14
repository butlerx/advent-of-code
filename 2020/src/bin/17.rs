use std::collections::{HashMap, HashSet};
use std::hash::Hash;

type Row3d = (i8, i8, i8);
type Row4d = (i8, i8, i8, i8);
static NEIGHBOURS: [Row4d; 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

fn main() {
    let input = include_str!("../../input/17.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn simulate<Pos: Hash + Eq + Copy, F: Fn(&HashSet<Pos>) -> HashMap<Pos, usize>>(
    active: HashSet<Pos>,
    count_neighbours: F,
) -> usize {
    (0..6)
        .fold(active, |active, _| {
            count_neighbours(&active)
                .iter()
                .filter(|(pos, n)| matches!((n, active.contains(pos)), (2, true) | (3, _)))
                .map(|(&pos, _)| pos)
                .collect()
        })
        .len()
}

fn count_neighbours_3d(active: &HashSet<Row3d>) -> HashMap<Row3d, usize> {
    let mut neighbours = HashMap::new();
    for (x, y, z) in active {
        for (_, dx, dy, dz) in &NEIGHBOURS[26..52] {
            *neighbours.entry((x + dx, y + dy, z + dz)).or_insert(0) += 1;
        }
    }
    neighbours
}
fn count_neighbours_4d(active: &HashSet<Row4d>) -> HashMap<Row4d, usize> {
    let mut neighbours = HashMap::new();
    for (x, y, z, w) in active {
        for (dx, dy, dz, dw) in &NEIGHBOURS {
            *neighbours
                .entry((x + dx, y + dy, z + dz, w + dw))
                .or_insert(0) += 1;
        }
    }
    neighbours
}

fn run(input: &str, part_two: bool) -> i64 {
    if part_two {
        let active = input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|&(_, b)| b == '#')
                    .map(move |(x, _)| (x as i8, y as i8, 0, 0))
            })
            .collect();
        simulate(active, count_neighbours_4d) as i64
    } else {
        let active = input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|&(_, b)| b == '#')
                    .map(move |(x, _)| (x as i8, y as i8, 0))
            })
            .collect();
        simulate(active, count_neighbours_3d) as i64
    }
}

#[cfg(test)]
mod dat_17_tests {
    use super::*;
    static INPUT: &str = ".#.
..#
###";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 112);
        let results = run(include_str!("../../input/17.txt"), false);
        println!("{}", results);
        assert!(results == 310);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 848);
        let results = run(include_str!("../../input/17.txt"), true);
        println!("{}", results);
        assert!(results == 2056);
    }
}
