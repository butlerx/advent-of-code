#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/08.txt");

fn main() {
    println!("🌟 --- Day 8 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn part_1(input: &str) -> usize {
    let coords = parse_coordinates(input);

    let edge_threshold = if coords.len() < 100 {
        coords.len() / 2
    } else {
        1000
    };

    compute_sorted_distances(&coords)
        .iter()
        .take(edge_threshold)
        .fold(UnionFind::new(coords.len()), |mut uf, &(_, i, j)| {
            uf.union(i, j);
            uf
        })
        .component_sizes()
        .into_iter()
        .filter(|&size| size > 0)
        .fold([0; 3], |mut top3, size| {
            if size > top3[0] {
                top3[2] = top3[1];
                top3[1] = top3[0];
                top3[0] = size;
            } else if size > top3[1] {
                top3[2] = top3[1];
                top3[1] = size;
            } else if size > top3[2] {
                top3[2] = size;
            }
            top3
        })
        .into_iter()
        .product()
}

fn part_2(input: &str) -> i64 {
    let coords = parse_coordinates(input);
    compute_sorted_distances(&coords)
        .iter()
        .scan(UnionFind::new(coords.len()), |uf, &(_, i, j)| {
            uf.union(i, j);
            Some((uf.is_fully_connected(), i, j))
        })
        .find_map(|(connected, i, j)| connected.then(|| coords[i].x * coords[j].x))
        .expect("Should find fully connected state")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    #[inline]
    fn squared_distance(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl From<&str> for Coordinate {
    fn from(s: &str) -> Self {
        let mut nums = s
            .trim()
            .split(',')
            .map(|part| part.parse().expect("invalid integer"));
        Self {
            x: nums.next().expect("missing x"),
            y: nums.next().expect("missing y"),
            z: nums.next().expect("missing z"),
        }
    }
}

fn parse_coordinates(input: &str) -> Vec<Coordinate> {
    input.trim().lines().map(Coordinate::from).collect()
}

fn compute_sorted_distances(coords: &[Coordinate]) -> Vec<(i64, usize, usize)> {
    let mut distances: Vec<_> = (0..coords.len())
        .flat_map(|i| {
            ((i + 1)..coords.len()).map(move |j| (coords[i].squared_distance(&coords[j]), i, j))
        })
        .collect();

    distances.sort_unstable_by_key(|&(d, _, _)| d);
    distances
}

struct UnionFind {
    parent: Vec<usize>,
    size: usize,
    num_components: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size,
            num_components: size,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            false
        } else {
            self.parent[root_a] = root_b;
            self.num_components -= 1;
            true
        }
    }

    fn is_fully_connected(&self) -> bool {
        self.num_components == 1
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        (0..self.size).fold(vec![0; self.size], |mut sizes, i| {
            let root = self.find(i);
            sizes[root] += 1;
            sizes
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 40);
        assert_eq!(part_1(INPUT_TXT), 153_328);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 25272);
        assert_eq!(part_2(INPUT_TXT), 6_095_621_910);
    }
}
