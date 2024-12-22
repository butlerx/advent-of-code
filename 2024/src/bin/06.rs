#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::{time_execution, Grid, Point};
use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/06.txt");

fn main() {
    println!("🌟 --- Day 6 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    #[inline]
    fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn as_delta(self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::East => Point::new(1, 0),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
        }
    }
}

fn parse_input(input: &str) -> (Grid<bool>, Point, Direction) {
    let cells = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(row, line)| {
            line.chars()
                .position(|c| c == '^')
                .map(|col| Point::from((col, row)))
        })
        .expect("no start position found");
    (Grid::from(cells), start_pos, Direction::North)
}

fn next_position(grid: &Grid<bool>, pos: Point, dir: Direction) -> Option<Point> {
    let next = pos + dir.as_delta();
    grid.in_bounds(next).then_some(next)
}

fn simulate_path(grid: &Grid<bool>, start: Point, mut dir: Direction) -> HashSet<Point> {
    let mut visited = HashSet::with_capacity(grid.height * grid.width);
    let mut current = start;

    loop {
        visited.insert(current);
        match next_position(grid, current, dir) {
            Some(next) if grid.get(next) == Some(true) => dir = dir.turn_right(),
            Some(next) => current = next,
            None => break visited,
        }
    }
}

fn part_1(input: &str) -> usize {
    let (grid, start, dir) = parse_input(input);
    simulate_path(&grid, start, dir).len()
}

fn part_2(input: &str) -> usize {
    let (grid, start, start_dir) = parse_input(input);

    let mut test_grid = grid.clone();
    let mut visited = HashSet::with_capacity(grid.height * grid.width);
    let mut collision_map = HashMap::with_capacity(grid.height * grid.width);

    simulate_path(&grid, start, start_dir)
        .into_iter()
        .filter(|&pos| pos != start)
        .filter(|&pos| {
            visited.clear();
            collision_map.clear();
            test_grid.set(pos, true);

            let mut current = (start, start_dir);

            while visited.insert(current) {
                let mut next = current.0;
                while let Some(pos) = next_position(&test_grid, next, current.1) {
                    if test_grid.get(pos) == Some(true) {
                        let next_state = (next, current.1.turn_right());
                        collision_map.insert(current, next_state);
                        current = next_state;
                        break;
                    }
                    next = pos;
                }
            }
            visited.clear();
            test_grid.set(pos, false);

            loop {
                if !visited.insert(current) {
                    break true;
                }
                match collision_map.get(&current) {
                    Some(&next) => current = next,
                    None => break false,
                }
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 41);
        assert_eq!(part_1(INPUT_TXT), 5409);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 6);
        assert_eq!(part_2(INPUT_TXT), 2022);
    }
}
