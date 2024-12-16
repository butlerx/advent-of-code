#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::{time_execution, Grid, Point};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

static INPUT_TXT: &str = include_str!("../../input/16.txt");

fn main() {
    println!("🌟 --- Day 16 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

const UP: Point = Point::new(0, -1);
const DOWN: Point = Point::new(0, 1);
const LEFT: Point = Point::new(-1, 0);
const RIGHT: Point = Point::new(1, 0);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn all() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    fn rotate(self) -> impl Iterator<Item = Self> {
        match self {
            Direction::North | Direction::South => {
                vec![Direction::East, Direction::West].into_iter()
            }
            Direction::West | Direction::East => {
                vec![Direction::North, Direction::South].into_iter()
            }
        }
    }

    fn move_point(self) -> Point {
        match self {
            Direction::North => UP,
            Direction::East => RIGHT,
            Direction::South => DOWN,
            Direction::West => LEFT,
        }
    }
}

type State = (Point, Direction);

fn find_next_states(
    grid: &Grid<char>,
    (point, dir): State,
    cost: i32,
    visited: &HashMap<State, i32>,
) -> Vec<(i32, State)> {
    let mut states = Vec::new();

    let new_point = point + dir.move_point();
    if grid.in_bounds(new_point) && grid.get(new_point) != Some('#') {
        let new_cost = cost + 1;
        if new_cost < *visited.get(&(new_point, dir)).unwrap_or(&i32::MAX) {
            states.push((new_cost, (new_point, dir)));
        }
    }

    states.extend(
        dir.rotate()
            .map(|new_dir| (cost + 1000, new_dir))
            .filter(|&(new_cost, new_dir)| {
                new_cost < *visited.get(&(point, new_dir)).unwrap_or(&i32::MAX)
            })
            .map(|(new_cost, new_dir)| (new_cost, (point, new_dir))),
    );

    states
}

fn dijkstra(grid: &Grid<char>, start: Point) -> HashMap<State, i32> {
    let mut visited = HashMap::from([((start, Direction::East), 0)]);
    let mut queue = BinaryHeap::from([Reverse((0, (start, Direction::East)))]);

    while let Some(Reverse((cost, current))) = queue.pop() {
        if cost > *visited.get(&current).unwrap_or(&i32::MAX) {
            continue;
        }

        for (new_cost, new_state) in find_next_states(grid, current, cost, &visited) {
            visited.insert(new_state, new_cost);
            queue.push(Reverse((new_cost, new_state)));
        }
    }

    visited
}

fn find_shortest_path(visited: &HashMap<State, i32>, end: Point) -> Option<i32> {
    Direction::all()
        .iter()
        .filter_map(|&d| visited.get(&(end, d)))
        .min()
        .copied()
}

fn find_previous_states(
    grid: &Grid<char>,
    (point, dir): State,
    visited: &HashMap<State, i32>,
) -> Vec<State> {
    let current_cost = visited[&(point, dir)];
    let mut states = Vec::new();

    let prev_point = point - dir.move_point();
    if grid.in_bounds(prev_point) && grid.get(prev_point) != Some('#') {
        let prev_cost = current_cost - 1;
        if prev_cost >= 0 && visited.get(&(prev_point, dir)) == Some(&prev_cost) {
            states.push((prev_point, dir));
        }
    }

    let turn_cost = current_cost - 1000;
    if turn_cost >= 0 {
        states.extend(
            dir.rotate()
                .filter(|&prev_dir| visited.get(&(point, prev_dir)) == Some(&turn_cost))
                .map(|prev_dir| (point, prev_dir)),
        );
    }

    states
}
fn backtrack_shortest_paths(
    grid: &Grid<char>,
    visited: &HashMap<State, i32>,
    end: Point,
) -> HashSet<Point> {
    let min_cost = find_shortest_path(visited, end).expect("no path found");
    let initial_states: HashSet<_> = Direction::all()
        .iter()
        .filter(|&&d| visited.get(&(end, d)) == Some(&min_cost))
        .map(|&d| (end, d))
        .collect();

    let mut result = HashSet::new();
    let mut queue = VecDeque::from_iter(initial_states);
    let mut seen = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if !seen.insert(state) {
            continue;
        }
        result.insert(state.0);

        for prev_state in find_previous_states(grid, state, visited) {
            if !seen.contains(&prev_state) {
                queue.push_back(prev_state);
            }
        }
    }

    result
}

fn parse_input(input: &str) -> (Grid<char>, Point, Point) {
    let grid = Grid::from(
        input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    );
    let start = grid.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let end = grid.iter().find(|(_, c)| **c == 'E').unwrap().0;
    (grid, start, end)
}

fn part_1(input: &str) -> i32 {
    let (grid, start, end) = parse_input(input);
    let visited = dijkstra(&grid, start);
    find_shortest_path(&visited, end).expect("no path found")
}

fn part_2(input: &str) -> usize {
    let (grid, start, end) = parse_input(input);
    let visited = dijkstra(&grid, start);
    backtrack_shortest_paths(&grid, &visited, end).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    static INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 7036);
        assert_eq!(part_1(INPUT_2), 11048);
        assert_eq!(part_1(INPUT_TXT), 83432);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 45);
        assert_eq!(part_2(INPUT_2), 64);
        assert_eq!(part_2(INPUT_TXT), 467);
    }
}
