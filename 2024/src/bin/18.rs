#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::{Grid, Point, time_execution};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

static INPUT_TXT: &str = include_str!("../../input/18.txt");

fn main() {
    println!("🌟 --- Day 18 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT, 1024, Point::new(70, 70)));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT, Point::new(70, 70)));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

const DIRECTIONS: [Point; 4] = [
    Point::new(1, 0),
    Point::new(0, 1),
    Point::new(-1, 0),
    Point::new(0, -1),
];

fn neighbors(point: Point) -> impl Iterator<Item = Point> {
    DIRECTIONS.iter().map(move |d| point + *d)
}

fn create_grid(bytes: &[Point], bytes_to_simulate: usize, exit: Point) -> Grid<bool> {
    bytes
        .iter()
        .take(bytes_to_simulate)
        .fold(Grid::new(exit, true), |mut grid, byte| {
            grid.set(*byte, false);
            grid
        })
}

fn is_valid_move(point: Point, grid: &Grid<bool>, explored: &HashSet<Point>) -> bool {
    !explored.contains(&point) && grid.get(point).unwrap_or(false)
}

fn explore_step(
    to_explore: &mut BinaryHeap<(Reverse<i64>, i64, Point)>,
    explored: &mut HashSet<Point>,
    grid: &Grid<bool>,
    exit: Point,
) -> Option<i64> {
    let (_, distance, current) = to_explore.pop()?;

    if current == exit {
        return Some(distance);
    }

    if explored.insert(current) {
        for neighbor in neighbors(current).filter(|n| is_valid_move(*n, grid, explored)) {
            let priority = distance + 1 + neighbor.manhattan_distance(exit);
            to_explore.push((Reverse(priority), distance + 1, neighbor));
        }
    }

    None
}

fn explore_at_time(bytes: &[Point], bytes_to_simulate: usize, exit: Point) -> Option<i64> {
    let grid = create_grid(bytes, bytes_to_simulate, exit);
    let start = Point::new(0, 0);
    let mut to_explore = BinaryHeap::from([(Reverse(start.manhattan_distance(exit)), 0, start)]);
    let mut explored = HashSet::new();
    std::iter::from_fn(|| {
        if to_explore.is_empty() {
            None
        } else {
            Some(explore_step(&mut to_explore, &mut explored, &grid, exit))
        }
    })
    .find_map(|x| x)
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| match line.split_once(',') {
            Some((x, y)) => Some(Point::new(x.parse().ok()?, y.parse().ok()?)),
            None => None,
        })
        .collect()
}

fn part_1(input: &str, bytes_to_simulate: usize, exit: Point) -> i64 {
    let bytes = parse_input(input);
    explore_at_time(&bytes, bytes_to_simulate, exit).expect("No path found")
}

fn find_last_possible(bytes: &[Point], exit: Point) -> usize {
    let mut left = 0;
    let mut right = bytes.len() - 1;

    while left < right {
        let mid = left + (right - left).div_ceil(2);
        if explore_at_time(bytes, mid, exit).is_some() {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn part_2(input: &str, exit: Point) -> String {
    let bytes = parse_input(input);
    bytes[find_last_possible(&bytes, exit)].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT, 12, Point::new(6, 6)), 22);
        assert_eq!(part_1(INPUT_TXT, 1024, Point::new(70, 70)), 322);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT, Point::new(6, 6)), "6,1");
        assert_eq!(part_2(INPUT_TXT, Point::new(70, 70)), "60,21");
    }
}
