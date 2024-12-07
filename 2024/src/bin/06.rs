use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/06.txt");

fn main() {
    println!("🌟 --- Day 6 Results --- 🌟");
    println!("📌 Part 1: {}", part_1(INPUT_TXT));
    println!("📌 Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    #[inline(always)]
    fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<bool>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> (Self, Position, Direction) {
        let cols = input.lines().next().unwrap().len();
        let cells: Vec<bool> = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c == '#')
            .collect();
        let rows = cells.len() / cols;

        let start_pos = input
            .lines()
            .enumerate()
            .find_map(|(row, line)| {
                line.chars()
                    .position(|c| c == '^')
                    .map(|col| Position { row, col })
            })
            .expect("no start position found");

        (Self { cells, rows, cols }, start_pos, Direction::North)
    }

    #[inline(always)]
    fn is_wall(&self, pos: Position) -> bool {
        self.cells[pos.row * self.cols + pos.col]
    }

    #[inline(always)]
    fn set_wall(&mut self, pos: Position, is_wall: bool) {
        self.cells[pos.row * self.cols + pos.col] = is_wall;
    }

    #[inline(always)]
    fn next_position(&self, pos: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::North if pos.row > 0 => Some(Position {
                row: pos.row - 1,
                col: pos.col,
            }),
            Direction::East if pos.col < self.cols - 1 => Some(Position {
                row: pos.row,
                col: pos.col + 1,
            }),
            Direction::South if pos.row < self.rows - 1 => Some(Position {
                row: pos.row + 1,
                col: pos.col,
            }),
            Direction::West if pos.col > 0 => Some(Position {
                row: pos.row,
                col: pos.col - 1,
            }),
            _ => None,
        }
    }

    fn simulate_path(&self, start: Position, mut dir: Direction) -> HashSet<Position> {
        let mut visited = HashSet::with_capacity(self.rows * self.cols);
        let mut current = start;

        loop {
            visited.insert(current);
            match self.next_position(current, dir) {
                Some(next) if self.is_wall(next) => dir = dir.turn_right(),
                Some(next) => current = next,
                None => break visited,
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    let (grid, start, dir) = Grid::new(input);
    grid.simulate_path(start, dir).len()
}

fn part_2(input: &str) -> usize {
    let (grid, start, start_dir) = Grid::new(input);

    let mut test_grid = grid.clone();
    let mut visited = HashSet::with_capacity(grid.rows * grid.cols);
    let mut collision_map = HashMap::with_capacity(grid.rows * grid.cols);

    grid.simulate_path(start, start_dir)
        .into_iter()
        .filter(|&pos| pos != start)
        .filter(|&pos| {
            visited.clear();
            collision_map.clear();
            test_grid.set_wall(pos, true);

            let mut current = (start, start_dir);

            while visited.insert(current) {
                let mut next = current.0;
                while let Some(pos) = test_grid.next_position(next, current.1) {
                    if test_grid.is_wall(pos) {
                        let next_state = (next, current.1.turn_right());
                        collision_map.insert(current, next_state);
                        current = next_state;
                        break;
                    }
                    next = pos;
                }
            }
            visited.clear();
            test_grid.set_wall(pos, false);

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
